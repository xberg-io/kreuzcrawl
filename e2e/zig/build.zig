const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const test_step = b.step("test", "Run tests");
    const ffi_path = b.option([]const u8, "ffi_path", "Path to directory containing libkreuzcrawl_ffi") orelse "../../target/release";
    const ffi_include = b.option([]const u8, "ffi_include_path", "Path to directory containing FFI header") orelse "../../crates/kreuzcrawl-ffi/include";

    const kreuzcrawl_module = b.addModule("kreuzcrawl", .{
        .root_source_file = b.path("../../packages/zig/src/kreuzcrawl.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    kreuzcrawl_module.addLibraryPath(.{ .cwd_relative = ffi_path });
    kreuzcrawl_module.addIncludePath(.{ .cwd_relative = ffi_include });
    kreuzcrawl_module.linkSystemLibrary("kreuzcrawl_ffi", .{});

    const _alloc = b.allocator;
    var mock_server_url: ?[]const u8 = b.graph.environ_map.get("MOCK_SERVER_URL");
    var mock_servers_json: ?[]const u8 = null;
    var mock_servers_map = std.StringHashMap([]const u8).init(_alloc);
    if (mock_server_url == null) {
        const _bin = b.pathFromRoot("../rust/target/release/mock-server");
        const _fixtures = b.pathFromRoot("../../fixtures");
        var _threaded = std.Io.Threaded.init(_alloc, .{});
        const _io = _threaded.io();
        const _spawned = std.process.spawn(_io, .{
            .argv = &.{ _bin, _fixtures },
            .stdin = .pipe,
            .stdout = .pipe,
            .stderr = .inherit,
        });
        if (_spawned) |_child| {
            // The child is intentionally not awaited: it lives for the duration
            // of the `zig build` process, which spans test execution.
            const _stdout = _child.stdout.?;
            var _buf: [65536]u8 = undefined;
            var _file_reader = _stdout.readerStreaming(_io, &_buf);
            const _r = &_file_reader.interface;
            // Read startup lines: MOCK_SERVER_URL= then MOCK_SERVERS= (always
            // emitted, possibly `{}`). Cap the loop so a misbehaving server
            // cannot block the build indefinitely.
            var _saw_url = false;
            var _i: usize = 0;
            while (_i < 64) : (_i += 1) {
                const _line_raw = _r.takeDelimiterExclusive('\n') catch break;
                const _line = std.mem.trim(u8, _line_raw, " \r\t");
                if (std.mem.startsWith(u8, _line, "MOCK_SERVER_URL=")) {
                    mock_server_url = _alloc.dupe(u8, _line["MOCK_SERVER_URL=".len..]) catch null;
                    _saw_url = true;
                } else if (std.mem.startsWith(u8, _line, "MOCK_SERVERS=")) {
                    const _json = _line["MOCK_SERVERS=".len..];
                    mock_servers_json = _alloc.dupe(u8, _json) catch null;
                    if (std.json.parseFromSlice(std.json.Value, _alloc, _json, .{})) |_parsed| {
                        if (_parsed.value == .object) {
                            var _entries = _parsed.value.object.iterator();
                            while (_entries.next()) |_entry| {
                                if (_entry.value_ptr.* == .string) {
                                    const _key = std.fmt.allocPrint(_alloc, "MOCK_SERVER_{s}", .{_entry.key_ptr.*}) catch continue;
                                    for (_key) |*_c| _c.* = std.ascii.toUpper(_c.*);
                                    const _val = _alloc.dupe(u8, _entry.value_ptr.*.string) catch continue;
                                    mock_servers_map.put(_key, _val) catch {};
                                }
                            }
                        }
                    } else |_| {}
                    break;
                } else if (_saw_url) {
                    break;
                }
            }
        } else |_| {
            // Binary not built — leave mock_server_url null so tests surface a
            // clear connection error rather than a build failure.
        }
    }

    const auth_module = b.createModule(.{
        .root_source_file = b.path("src/auth_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    auth_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const auth_tests = b.addTest(.{
        .name = "auth_test",
        .root_module = auth_module,
        .use_llvm = true,
    });
    const auth_run = b.addRunArtifact(auth_tests);
    if (mock_server_url) |_url| {
        auth_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        auth_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            auth_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&auth_run.step);

    const browser_module = b.createModule(.{
        .root_source_file = b.path("src/browser_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    browser_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const browser_tests = b.addTest(.{
        .name = "browser_test",
        .root_module = browser_module,
        .use_llvm = true,
    });
    const browser_run = b.addRunArtifact(browser_tests);
    if (mock_server_url) |_url| {
        browser_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        browser_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            browser_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&browser_run.step);

    const cache_module = b.createModule(.{
        .root_source_file = b.path("src/cache_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    cache_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const cache_tests = b.addTest(.{
        .name = "cache_test",
        .root_module = cache_module,
        .use_llvm = true,
    });
    const cache_run = b.addRunArtifact(cache_tests);
    if (mock_server_url) |_url| {
        cache_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        cache_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            cache_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&cache_run.step);

    const concurrent_module = b.createModule(.{
        .root_source_file = b.path("src/concurrent_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    concurrent_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const concurrent_tests = b.addTest(.{
        .name = "concurrent_test",
        .root_module = concurrent_module,
        .use_llvm = true,
    });
    const concurrent_run = b.addRunArtifact(concurrent_tests);
    if (mock_server_url) |_url| {
        concurrent_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        concurrent_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            concurrent_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&concurrent_run.step);

    const content_module = b.createModule(.{
        .root_source_file = b.path("src/content_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    content_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const content_tests = b.addTest(.{
        .name = "content_test",
        .root_module = content_module,
        .use_llvm = true,
    });
    const content_run = b.addRunArtifact(content_tests);
    if (mock_server_url) |_url| {
        content_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        content_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            content_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&content_run.step);

    const cookies_module = b.createModule(.{
        .root_source_file = b.path("src/cookies_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    cookies_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const cookies_tests = b.addTest(.{
        .name = "cookies_test",
        .root_module = cookies_module,
        .use_llvm = true,
    });
    const cookies_run = b.addRunArtifact(cookies_tests);
    if (mock_server_url) |_url| {
        cookies_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        cookies_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            cookies_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&cookies_run.step);

    const crawl_module = b.createModule(.{
        .root_source_file = b.path("src/crawl_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    crawl_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const crawl_tests = b.addTest(.{
        .name = "crawl_test",
        .root_module = crawl_module,
        .use_llvm = true,
    });
    const crawl_run = b.addRunArtifact(crawl_tests);
    if (mock_server_url) |_url| {
        crawl_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        crawl_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            crawl_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&crawl_run.step);

    const download_module = b.createModule(.{
        .root_source_file = b.path("src/download_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    download_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const download_tests = b.addTest(.{
        .name = "download_test",
        .root_module = download_module,
        .use_llvm = true,
    });
    const download_run = b.addRunArtifact(download_tests);
    if (mock_server_url) |_url| {
        download_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        download_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            download_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&download_run.step);

    const encoding_module = b.createModule(.{
        .root_source_file = b.path("src/encoding_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    encoding_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const encoding_tests = b.addTest(.{
        .name = "encoding_test",
        .root_module = encoding_module,
        .use_llvm = true,
    });
    const encoding_run = b.addRunArtifact(encoding_tests);
    if (mock_server_url) |_url| {
        encoding_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        encoding_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            encoding_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&encoding_run.step);

    const engine_module = b.createModule(.{
        .root_source_file = b.path("src/engine_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    engine_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const engine_tests = b.addTest(.{
        .name = "engine_test",
        .root_module = engine_module,
        .use_llvm = true,
    });
    const engine_run = b.addRunArtifact(engine_tests);
    if (mock_server_url) |_url| {
        engine_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        engine_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            engine_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&engine_run.step);

    const error_module = b.createModule(.{
        .root_source_file = b.path("src/error_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    error_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const error_tests = b.addTest(.{
        .name = "error_test",
        .root_module = error_module,
        .use_llvm = true,
    });
    const error_run = b.addRunArtifact(error_tests);
    if (mock_server_url) |_url| {
        error_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        error_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            error_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&error_run.step);

    const filter_module = b.createModule(.{
        .root_source_file = b.path("src/filter_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    filter_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const filter_tests = b.addTest(.{
        .name = "filter_test",
        .root_module = filter_module,
        .use_llvm = true,
    });
    const filter_run = b.addRunArtifact(filter_tests);
    if (mock_server_url) |_url| {
        filter_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        filter_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            filter_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&filter_run.step);

    const interaction_module = b.createModule(.{
        .root_source_file = b.path("src/interaction_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    interaction_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const interaction_tests = b.addTest(.{
        .name = "interaction_test",
        .root_module = interaction_module,
        .use_llvm = true,
    });
    const interaction_run = b.addRunArtifact(interaction_tests);
    if (mock_server_url) |_url| {
        interaction_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        interaction_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            interaction_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&interaction_run.step);

    const links_module = b.createModule(.{
        .root_source_file = b.path("src/links_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    links_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const links_tests = b.addTest(.{
        .name = "links_test",
        .root_module = links_module,
        .use_llvm = true,
    });
    const links_run = b.addRunArtifact(links_tests);
    if (mock_server_url) |_url| {
        links_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        links_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            links_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&links_run.step);

    const map_module = b.createModule(.{
        .root_source_file = b.path("src/map_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    map_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const map_tests = b.addTest(.{
        .name = "map_test",
        .root_module = map_module,
        .use_llvm = true,
    });
    const map_run = b.addRunArtifact(map_tests);
    if (mock_server_url) |_url| {
        map_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        map_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            map_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&map_run.step);

    const markdown_module = b.createModule(.{
        .root_source_file = b.path("src/markdown_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    markdown_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const markdown_tests = b.addTest(.{
        .name = "markdown_test",
        .root_module = markdown_module,
        .use_llvm = true,
    });
    const markdown_run = b.addRunArtifact(markdown_tests);
    if (mock_server_url) |_url| {
        markdown_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        markdown_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            markdown_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&markdown_run.step);

    const metadata_module = b.createModule(.{
        .root_source_file = b.path("src/metadata_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    metadata_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const metadata_tests = b.addTest(.{
        .name = "metadata_test",
        .root_module = metadata_module,
        .use_llvm = true,
    });
    const metadata_run = b.addRunArtifact(metadata_tests);
    if (mock_server_url) |_url| {
        metadata_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        metadata_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            metadata_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&metadata_run.step);

    const proxy_module = b.createModule(.{
        .root_source_file = b.path("src/proxy_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    proxy_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const proxy_tests = b.addTest(.{
        .name = "proxy_test",
        .root_module = proxy_module,
        .use_llvm = true,
    });
    const proxy_run = b.addRunArtifact(proxy_tests);
    if (mock_server_url) |_url| {
        proxy_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        proxy_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            proxy_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&proxy_run.step);

    const rate_limit_module = b.createModule(.{
        .root_source_file = b.path("src/rate_limit_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    rate_limit_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const rate_limit_tests = b.addTest(.{
        .name = "rate_limit_test",
        .root_module = rate_limit_module,
        .use_llvm = true,
    });
    const rate_limit_run = b.addRunArtifact(rate_limit_tests);
    if (mock_server_url) |_url| {
        rate_limit_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        rate_limit_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            rate_limit_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&rate_limit_run.step);

    const redirect_module = b.createModule(.{
        .root_source_file = b.path("src/redirect_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    redirect_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const redirect_tests = b.addTest(.{
        .name = "redirect_test",
        .root_module = redirect_module,
        .use_llvm = true,
    });
    const redirect_run = b.addRunArtifact(redirect_tests);
    if (mock_server_url) |_url| {
        redirect_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        redirect_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            redirect_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&redirect_run.step);

    const robots_module = b.createModule(.{
        .root_source_file = b.path("src/robots_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    robots_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const robots_tests = b.addTest(.{
        .name = "robots_test",
        .root_module = robots_module,
        .use_llvm = true,
    });
    const robots_run = b.addRunArtifact(robots_tests);
    if (mock_server_url) |_url| {
        robots_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        robots_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            robots_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&robots_run.step);

    const scrape_module = b.createModule(.{
        .root_source_file = b.path("src/scrape_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    scrape_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const scrape_tests = b.addTest(.{
        .name = "scrape_test",
        .root_module = scrape_module,
        .use_llvm = true,
    });
    const scrape_run = b.addRunArtifact(scrape_tests);
    if (mock_server_url) |_url| {
        scrape_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        scrape_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            scrape_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&scrape_run.step);

    const sitemap_module = b.createModule(.{
        .root_source_file = b.path("src/sitemap_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    sitemap_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const sitemap_tests = b.addTest(.{
        .name = "sitemap_test",
        .root_module = sitemap_module,
        .use_llvm = true,
    });
    const sitemap_run = b.addRunArtifact(sitemap_tests);
    if (mock_server_url) |_url| {
        sitemap_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        sitemap_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            sitemap_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&sitemap_run.step);

    const stealth_module = b.createModule(.{
        .root_source_file = b.path("src/stealth_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    stealth_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const stealth_tests = b.addTest(.{
        .name = "stealth_test",
        .root_module = stealth_module,
        .use_llvm = true,
    });
    const stealth_run = b.addRunArtifact(stealth_tests);
    if (mock_server_url) |_url| {
        stealth_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        stealth_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            stealth_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&stealth_run.step);

    const strategy_module = b.createModule(.{
        .root_source_file = b.path("src/strategy_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    strategy_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const strategy_tests = b.addTest(.{
        .name = "strategy_test",
        .root_module = strategy_module,
        .use_llvm = true,
    });
    const strategy_run = b.addRunArtifact(strategy_tests);
    if (mock_server_url) |_url| {
        strategy_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        strategy_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            strategy_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&strategy_run.step);

    const validation_module = b.createModule(.{
        .root_source_file = b.path("src/validation_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    validation_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const validation_tests = b.addTest(.{
        .name = "validation_test",
        .root_module = validation_module,
        .use_llvm = true,
    });
    const validation_run = b.addRunArtifact(validation_tests);
    if (mock_server_url) |_url| {
        validation_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        validation_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            validation_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&validation_run.step);

    const warc_module = b.createModule(.{
        .root_source_file = b.path("src/warc_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    warc_module.addImport("kreuzcrawl", kreuzcrawl_module);
    const warc_tests = b.addTest(.{
        .name = "warc_test",
        .root_module = warc_module,
        .use_llvm = true,
    });
    const warc_run = b.addRunArtifact(warc_tests);
    if (mock_server_url) |_url| {
        warc_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        warc_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            warc_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&warc_run.step);

}
