import RustBridgeC

public func generateCitations<GenericIntoRustString: IntoRustString>(
  _ markdown: GenericIntoRustString
) -> CitationResult {
  CitationResult(
    ptr: __swift_bridge__$generate_citations(
      {
        let rustString = markdown.intoRustString()
        rustString.isOwned = false
        return rustString.ptr
      }()))
}
public func createEngine(_ config: CrawlConfig?) throws -> CrawlEngineHandle {
  try {
    let val = __swift_bridge__$create_engine(
      {
        if let val = config {
          val.isOwned = false
          return val.ptr
        } else {
          return nil
        }
      }())
    if val.is_ok {
      return CrawlEngineHandle(ptr: val.ok_or_err!)
    } else {
      throw RustString(ptr: val.ok_or_err!)
    }
  }()
}
public func scrape<GenericIntoRustString: IntoRustString>(
  _ engine: CrawlEngineHandle, _ url: GenericIntoRustString
) throws -> ScrapeResult {
  try {
    let val = __swift_bridge__$scrape(
      {
        engine.isOwned = false
        return engine.ptr
      }(),
      {
        let rustString = url.intoRustString()
        rustString.isOwned = false
        return rustString.ptr
      }())
    if val.is_ok {
      return ScrapeResult(ptr: val.ok_or_err!)
    } else {
      throw RustString(ptr: val.ok_or_err!)
    }
  }()
}
public func crawl<GenericIntoRustString: IntoRustString>(
  _ engine: CrawlEngineHandle, _ url: GenericIntoRustString
) throws -> CrawlResult {
  try {
    let val = __swift_bridge__$crawl(
      {
        engine.isOwned = false
        return engine.ptr
      }(),
      {
        let rustString = url.intoRustString()
        rustString.isOwned = false
        return rustString.ptr
      }())
    if val.is_ok {
      return CrawlResult(ptr: val.ok_or_err!)
    } else {
      throw RustString(ptr: val.ok_or_err!)
    }
  }()
}
public func mapUrls<GenericIntoRustString: IntoRustString>(
  _ engine: CrawlEngineHandle, _ url: GenericIntoRustString
) throws -> MapResult {
  try {
    let val = __swift_bridge__$map_urls(
      {
        engine.isOwned = false
        return engine.ptr
      }(),
      {
        let rustString = url.intoRustString()
        rustString.isOwned = false
        return rustString.ptr
      }())
    if val.is_ok {
      return MapResult(ptr: val.ok_or_err!)
    } else {
      throw RustString(ptr: val.ok_or_err!)
    }
  }()
}
public func interact<GenericIntoRustString: IntoRustString>(
  _ engine: CrawlEngineHandle, _ url: GenericIntoRustString,
  _ actions: RustVec<GenericIntoRustString>
) throws -> InteractionResult {
  try {
    let val = __swift_bridge__$interact(
      {
        engine.isOwned = false
        return engine.ptr
      }(),
      {
        let rustString = url.intoRustString()
        rustString.isOwned = false
        return rustString.ptr
      }(),
      {
        let val = actions
        val.isOwned = false
        return val.ptr
      }())
    if val.is_ok {
      return InteractionResult(ptr: val.ok_or_err!)
    } else {
      throw RustString(ptr: val.ok_or_err!)
    }
  }()
}
public func batchScrape<GenericIntoRustString: IntoRustString>(
  _ engine: CrawlEngineHandle, _ urls: RustVec<GenericIntoRustString>
) throws -> RustVec<BatchScrapeResult> {
  try {
    let val = __swift_bridge__$batch_scrape(
      {
        engine.isOwned = false
        return engine.ptr
      }(),
      {
        let val = urls
        val.isOwned = false
        return val.ptr
      }())
    if val.is_ok {
      return RustVec(ptr: val.ok_or_err!)
    } else {
      throw RustString(ptr: val.ok_or_err!)
    }
  }()
}
public func batchCrawl<GenericIntoRustString: IntoRustString>(
  _ engine: CrawlEngineHandle, _ urls: RustVec<GenericIntoRustString>
) throws -> RustVec<BatchCrawlResult> {
  try {
    let val = __swift_bridge__$batch_crawl(
      {
        engine.isOwned = false
        return engine.ptr
      }(),
      {
        let val = urls
        val.isOwned = false
        return val.ptr
      }())
    if val.is_ok {
      return RustVec(ptr: val.ok_or_err!)
    } else {
      throw RustString(ptr: val.ok_or_err!)
    }
  }()
}
public func crawlConfigFromJson<GenericIntoRustString: IntoRustString>(
  _ json: GenericIntoRustString
) throws -> CrawlConfig {
  try {
    let val = __swift_bridge__$crawl_config_from_json(
      {
        let rustString = json.intoRustString()
        rustString.isOwned = false
        return rustString.ptr
      }())
    if val.is_ok {
      return CrawlConfig(ptr: val.ok_or_err!)
    } else {
      throw RustString(ptr: val.ok_or_err!)
    }
  }()
}

public class ExtractionMeta: ExtractionMetaRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ExtractionMeta$_free(ptr)
    }
  }
}
extension ExtractionMeta {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ cost: Double?, _ prompt_tokens: UInt64?, _ completion_tokens: UInt64?,
    _ model: GenericIntoRustString?, _ chunks_processed: UInt
  ) {
    self.init(
      ptr: __swift_bridge__$ExtractionMeta$new(
        cost.intoFfiRepr(), prompt_tokens.intoFfiRepr(), completion_tokens.intoFfiRepr(),
        {
          if let rustString = optionalStringIntoRustString(model) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(), chunks_processed))
  }
}
public class ExtractionMetaRefMut: ExtractionMetaRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ExtractionMetaRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ExtractionMetaRef {
  public func cost() -> Double? {
    __swift_bridge__$ExtractionMeta$cost(ptr).intoSwiftRepr()
  }

  public func prompt_tokens() -> UInt64? {
    __swift_bridge__$ExtractionMeta$prompt_tokens(ptr).intoSwiftRepr()
  }

  public func completion_tokens() -> UInt64? {
    __swift_bridge__$ExtractionMeta$completion_tokens(ptr).intoSwiftRepr()
  }

  public func model() -> RustString? {
    {
      let val = __swift_bridge__$ExtractionMeta$model(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func chunks_processed() -> UInt {
    __swift_bridge__$ExtractionMeta$chunks_processed(ptr)
  }
}
extension ExtractionMeta: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ExtractionMeta$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ExtractionMeta$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ExtractionMeta) {
    __swift_bridge__$Vec_ExtractionMeta$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_ExtractionMeta$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ExtractionMeta(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> ExtractionMetaRef?
  {
    let pointer = __swift_bridge__$Vec_ExtractionMeta$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ExtractionMetaRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> ExtractionMetaRefMut?
  {
    let pointer = __swift_bridge__$Vec_ExtractionMeta$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ExtractionMetaRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    ExtractionMetaRef
  > {
    UnsafePointer<ExtractionMetaRef>(
      OpaquePointer(__swift_bridge__$Vec_ExtractionMeta$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ExtractionMeta$len(vecPtr)
  }
}

public class ProxyConfig: ProxyConfigRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ProxyConfig$_free(ptr)
    }
  }
}
extension ProxyConfig {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ url: GenericIntoRustString, _ username: GenericIntoRustString?,
    _ password: GenericIntoRustString?
  ) {
    self.init(
      ptr: __swift_bridge__$ProxyConfig$new(
        {
          let rustString = url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(username) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(password) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }()))
  }
}
public class ProxyConfigRefMut: ProxyConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ProxyConfigRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ProxyConfigRef {
  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$ProxyConfig$url(ptr))
  }

  public func username() -> RustString? {
    {
      let val = __swift_bridge__$ProxyConfig$username(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func password() -> RustString? {
    {
      let val = __swift_bridge__$ProxyConfig$password(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }
}
extension ProxyConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ProxyConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ProxyConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ProxyConfig) {
    __swift_bridge__$Vec_ProxyConfig$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_ProxyConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ProxyConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> ProxyConfigRef? {
    let pointer = __swift_bridge__$Vec_ProxyConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ProxyConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> ProxyConfigRefMut?
  {
    let pointer = __swift_bridge__$Vec_ProxyConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ProxyConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    ProxyConfigRef
  > {
    UnsafePointer<ProxyConfigRef>(OpaquePointer(__swift_bridge__$Vec_ProxyConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ProxyConfig$len(vecPtr)
  }
}

public class ContentConfig: ContentConfigRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ContentConfig$_free(ptr)
    }
  }
}
extension ContentConfig {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ output_format: GenericIntoRustString, _ preprocessing_preset: GenericIntoRustString,
    _ remove_navigation: Bool, _ remove_forms: Bool, _ strip_tags: RustVec<GenericIntoRustString>,
    _ preserve_tags: RustVec<GenericIntoRustString>,
    _ exclude_selectors: RustVec<GenericIntoRustString>, _ skip_images: Bool, _ max_depth: UInt?,
    _ wrap: Bool, _ wrap_width: UInt, _ include_document_structure: Bool
  ) {
    self.init(
      ptr: __swift_bridge__$ContentConfig$new(
        {
          let rustString = output_format.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let rustString = preprocessing_preset.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(), remove_navigation, remove_forms,
        {
          let val = strip_tags
          val.isOwned = false
          return val.ptr
        }(),
        {
          let val = preserve_tags
          val.isOwned = false
          return val.ptr
        }(),
        {
          let val = exclude_selectors
          val.isOwned = false
          return val.ptr
        }(), skip_images, max_depth.intoFfiRepr(), wrap, wrap_width, include_document_structure))
  }
}
public class ContentConfigRefMut: ContentConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ContentConfigRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ContentConfigRef {
  public func output_format() -> RustString {
    RustString(ptr: __swift_bridge__$ContentConfig$output_format(ptr))
  }

  public func preprocessing_preset() -> RustString {
    RustString(ptr: __swift_bridge__$ContentConfig$preprocessing_preset(ptr))
  }

  public func remove_navigation() -> Bool {
    __swift_bridge__$ContentConfig$remove_navigation(ptr)
  }

  public func remove_forms() -> Bool {
    __swift_bridge__$ContentConfig$remove_forms(ptr)
  }

  public func strip_tags() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$ContentConfig$strip_tags(ptr))
  }

  public func preserve_tags() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$ContentConfig$preserve_tags(ptr))
  }

  public func exclude_selectors() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$ContentConfig$exclude_selectors(ptr))
  }

  public func skip_images() -> Bool {
    __swift_bridge__$ContentConfig$skip_images(ptr)
  }

  public func max_depth() -> UInt? {
    __swift_bridge__$ContentConfig$max_depth(ptr).intoSwiftRepr()
  }

  public func wrap() -> Bool {
    __swift_bridge__$ContentConfig$wrap(ptr)
  }

  public func wrap_width() -> UInt {
    __swift_bridge__$ContentConfig$wrap_width(ptr)
  }

  public func include_document_structure() -> Bool {
    __swift_bridge__$ContentConfig$include_document_structure(ptr)
  }
}
extension ContentConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ContentConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ContentConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ContentConfig) {
    __swift_bridge__$Vec_ContentConfig$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_ContentConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ContentConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> ContentConfigRef?
  {
    let pointer = __swift_bridge__$Vec_ContentConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ContentConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> ContentConfigRefMut?
  {
    let pointer = __swift_bridge__$Vec_ContentConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ContentConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    ContentConfigRef
  > {
    UnsafePointer<ContentConfigRef>(
      OpaquePointer(__swift_bridge__$Vec_ContentConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ContentConfig$len(vecPtr)
  }
}

public class BrowserConfig: BrowserConfigRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$BrowserConfig$_free(ptr)
    }
  }
}
extension BrowserConfig {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ mode: BrowserMode, _ backend: BrowserBackend, _ endpoint: GenericIntoRustString?,
    _ timeout: UInt64, _ wait: BrowserWait, _ wait_selector: GenericIntoRustString?,
    _ extra_wait: UInt64?, _ stealth: Bool, _ proxy: ProxyConfig?,
    _ block_url_patterns: RustVec<GenericIntoRustString>, _ eval_script: GenericIntoRustString?,
    _ robots_user_agent: GenericIntoRustString?, _ capture_network_events: Bool
  ) {
    self.init(
      ptr: __swift_bridge__$BrowserConfig$new(
        {
          mode.isOwned = false
          return mode.ptr
        }(),
        {
          backend.isOwned = false
          return backend.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(endpoint) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(), timeout,
        {
          wait.isOwned = false
          return wait.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(wait_selector) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(), extra_wait.intoFfiRepr(), stealth,
        {
          if let val = proxy {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          let val = block_url_patterns
          val.isOwned = false
          return val.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(eval_script) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(robots_user_agent) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(), capture_network_events))
  }
}
public class BrowserConfigRefMut: BrowserConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class BrowserConfigRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension BrowserConfigRef {
  public func mode() -> RustString {
    RustString(ptr: __swift_bridge__$BrowserConfig$mode(ptr))
  }

  public func backend() -> RustString {
    RustString(ptr: __swift_bridge__$BrowserConfig$backend(ptr))
  }

  public func endpoint() -> RustString? {
    {
      let val = __swift_bridge__$BrowserConfig$endpoint(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func timeout() -> UInt64 {
    __swift_bridge__$BrowserConfig$timeout(ptr)
  }

  public func wait() -> RustString {
    RustString(ptr: __swift_bridge__$BrowserConfig$wait(ptr))
  }

  public func wait_selector() -> RustString? {
    {
      let val = __swift_bridge__$BrowserConfig$wait_selector(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func extra_wait() -> UInt64? {
    __swift_bridge__$BrowserConfig$extra_wait(ptr).intoSwiftRepr()
  }

  public func stealth() -> Bool {
    __swift_bridge__$BrowserConfig$stealth(ptr)
  }

  public func proxy() -> ProxyConfig? {
    {
      let val = __swift_bridge__$BrowserConfig$proxy(ptr)
      if val != nil { return ProxyConfig(ptr: val!) } else { return nil }
    }()
  }

  public func block_url_patterns() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$BrowserConfig$block_url_patterns(ptr))
  }

  public func eval_script() -> RustString? {
    {
      let val = __swift_bridge__$BrowserConfig$eval_script(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func robots_user_agent() -> RustString? {
    {
      let val = __swift_bridge__$BrowserConfig$robots_user_agent(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func capture_network_events() -> Bool {
    __swift_bridge__$BrowserConfig$capture_network_events(ptr)
  }
}
extension BrowserConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_BrowserConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_BrowserConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BrowserConfig) {
    __swift_bridge__$Vec_BrowserConfig$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_BrowserConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (BrowserConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> BrowserConfigRef?
  {
    let pointer = __swift_bridge__$Vec_BrowserConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BrowserConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> BrowserConfigRefMut?
  {
    let pointer = __swift_bridge__$Vec_BrowserConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BrowserConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    BrowserConfigRef
  > {
    UnsafePointer<BrowserConfigRef>(
      OpaquePointer(__swift_bridge__$Vec_BrowserConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_BrowserConfig$len(vecPtr)
  }
}

public class CrawlConfig: CrawlConfigRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$CrawlConfig$_free(ptr)
    }
  }
}
extension CrawlConfig {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ max_depth: UInt?, _ max_pages: UInt?, _ max_concurrent: UInt?, _ respect_robots_txt: Bool,
    _ soft_http_errors: Bool, _ user_agent: GenericIntoRustString?, _ stay_on_domain: Bool,
    _ allow_subdomains: Bool, _ include_paths: RustVec<GenericIntoRustString>,
    _ exclude_paths: RustVec<GenericIntoRustString>, _ custom_headers: GenericIntoRustString,
    _ request_timeout: UInt64, _ rate_limit_ms: UInt64?, _ max_redirects: UInt, _ retry_count: UInt,
    _ retry_codes: RustVec<UInt16>, _ cookies_enabled: Bool, _ auth: AuthConfig?,
    _ max_body_size: UInt?, _ remove_tags: RustVec<GenericIntoRustString>, _ content: ContentConfig,
    _ map_limit: UInt?, _ map_search: GenericIntoRustString?, _ download_assets: Bool,
    _ asset_types: RustVec<AssetCategory>, _ max_asset_size: UInt?, _ browser: BrowserConfig,
    _ proxy: ProxyConfig?, _ user_agents: RustVec<GenericIntoRustString>,
    _ capture_screenshot: Bool, _ download_documents: Bool, _ document_max_size: UInt?,
    _ document_mime_types: RustVec<GenericIntoRustString>, _ warc_output: GenericIntoRustString?,
    _ browser_profile: GenericIntoRustString?, _ save_browser_profile: Bool
  ) {
    self.init(
      ptr: __swift_bridge__$CrawlConfig$new(
        max_depth.intoFfiRepr(), max_pages.intoFfiRepr(), max_concurrent.intoFfiRepr(),
        respect_robots_txt, soft_http_errors,
        {
          if let rustString = optionalStringIntoRustString(user_agent) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(), stay_on_domain, allow_subdomains,
        {
          let val = include_paths
          val.isOwned = false
          return val.ptr
        }(),
        {
          let val = exclude_paths
          val.isOwned = false
          return val.ptr
        }(),
        {
          let rustString = custom_headers.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(), request_timeout, rate_limit_ms.intoFfiRepr(), max_redirects, retry_count,
        {
          let val = retry_codes
          val.isOwned = false
          return val.ptr
        }(), cookies_enabled,
        {
          if let val = auth {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(), max_body_size.intoFfiRepr(),
        {
          let val = remove_tags
          val.isOwned = false
          return val.ptr
        }(),
        {
          content.isOwned = false
          return content.ptr
        }(), map_limit.intoFfiRepr(),
        {
          if let rustString = optionalStringIntoRustString(map_search) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(), download_assets,
        {
          let val = asset_types
          val.isOwned = false
          return val.ptr
        }(), max_asset_size.intoFfiRepr(),
        {
          browser.isOwned = false
          return browser.ptr
        }(),
        {
          if let val = proxy {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          let val = user_agents
          val.isOwned = false
          return val.ptr
        }(), capture_screenshot, download_documents, document_max_size.intoFfiRepr(),
        {
          let val = document_mime_types
          val.isOwned = false
          return val.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(warc_output) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(browser_profile) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(), save_browser_profile))
  }
}
public class CrawlConfigRefMut: CrawlConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class CrawlConfigRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension CrawlConfigRef {
  public func max_depth() -> UInt? {
    __swift_bridge__$CrawlConfig$max_depth(ptr).intoSwiftRepr()
  }

  public func max_pages() -> UInt? {
    __swift_bridge__$CrawlConfig$max_pages(ptr).intoSwiftRepr()
  }

  public func max_concurrent() -> UInt? {
    __swift_bridge__$CrawlConfig$max_concurrent(ptr).intoSwiftRepr()
  }

  public func respect_robots_txt() -> Bool {
    __swift_bridge__$CrawlConfig$respect_robots_txt(ptr)
  }

  public func soft_http_errors() -> Bool {
    __swift_bridge__$CrawlConfig$soft_http_errors(ptr)
  }

  public func user_agent() -> RustString? {
    {
      let val = __swift_bridge__$CrawlConfig$user_agent(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func stay_on_domain() -> Bool {
    __swift_bridge__$CrawlConfig$stay_on_domain(ptr)
  }

  public func allow_subdomains() -> Bool {
    __swift_bridge__$CrawlConfig$allow_subdomains(ptr)
  }

  public func include_paths() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$CrawlConfig$include_paths(ptr))
  }

  public func exclude_paths() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$CrawlConfig$exclude_paths(ptr))
  }

  public func custom_headers() -> RustString {
    RustString(ptr: __swift_bridge__$CrawlConfig$custom_headers(ptr))
  }

  public func request_timeout() -> UInt64 {
    __swift_bridge__$CrawlConfig$request_timeout(ptr)
  }

  public func rate_limit_ms() -> UInt64? {
    __swift_bridge__$CrawlConfig$rate_limit_ms(ptr).intoSwiftRepr()
  }

  public func max_redirects() -> UInt {
    __swift_bridge__$CrawlConfig$max_redirects(ptr)
  }

  public func retry_count() -> UInt {
    __swift_bridge__$CrawlConfig$retry_count(ptr)
  }

  public func retry_codes() -> RustVec<UInt16> {
    RustVec(ptr: __swift_bridge__$CrawlConfig$retry_codes(ptr))
  }

  public func cookies_enabled() -> Bool {
    __swift_bridge__$CrawlConfig$cookies_enabled(ptr)
  }

  public func auth() -> RustString? {
    {
      let val = __swift_bridge__$CrawlConfig$auth(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func max_body_size() -> UInt? {
    __swift_bridge__$CrawlConfig$max_body_size(ptr).intoSwiftRepr()
  }

  public func remove_tags() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$CrawlConfig$remove_tags(ptr))
  }

  public func content() -> ContentConfig {
    ContentConfig(ptr: __swift_bridge__$CrawlConfig$content(ptr))
  }

  public func map_limit() -> UInt? {
    __swift_bridge__$CrawlConfig$map_limit(ptr).intoSwiftRepr()
  }

  public func map_search() -> RustString? {
    {
      let val = __swift_bridge__$CrawlConfig$map_search(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func download_assets() -> Bool {
    __swift_bridge__$CrawlConfig$download_assets(ptr)
  }

  public func asset_types() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$CrawlConfig$asset_types(ptr))
  }

  public func max_asset_size() -> UInt? {
    __swift_bridge__$CrawlConfig$max_asset_size(ptr).intoSwiftRepr()
  }

  public func browser() -> BrowserConfig {
    BrowserConfig(ptr: __swift_bridge__$CrawlConfig$browser(ptr))
  }

  public func proxy() -> ProxyConfig? {
    {
      let val = __swift_bridge__$CrawlConfig$proxy(ptr)
      if val != nil { return ProxyConfig(ptr: val!) } else { return nil }
    }()
  }

  public func user_agents() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$CrawlConfig$user_agents(ptr))
  }

  public func capture_screenshot() -> Bool {
    __swift_bridge__$CrawlConfig$capture_screenshot(ptr)
  }

  public func download_documents() -> Bool {
    __swift_bridge__$CrawlConfig$download_documents(ptr)
  }

  public func document_max_size() -> UInt? {
    __swift_bridge__$CrawlConfig$document_max_size(ptr).intoSwiftRepr()
  }

  public func document_mime_types() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$CrawlConfig$document_mime_types(ptr))
  }

  public func warc_output() -> RustString? {
    {
      let val = __swift_bridge__$CrawlConfig$warc_output(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func browser_profile() -> RustString? {
    {
      let val = __swift_bridge__$CrawlConfig$browser_profile(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func save_browser_profile() -> Bool {
    __swift_bridge__$CrawlConfig$save_browser_profile(ptr)
  }
}
extension CrawlConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_CrawlConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_CrawlConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CrawlConfig) {
    __swift_bridge__$Vec_CrawlConfig$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_CrawlConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (CrawlConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> CrawlConfigRef? {
    let pointer = __swift_bridge__$Vec_CrawlConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CrawlConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> CrawlConfigRefMut?
  {
    let pointer = __swift_bridge__$Vec_CrawlConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CrawlConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    CrawlConfigRef
  > {
    UnsafePointer<CrawlConfigRef>(OpaquePointer(__swift_bridge__$Vec_CrawlConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_CrawlConfig$len(vecPtr)
  }
}

public class BrowserExtras: BrowserExtrasRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$BrowserExtras$_free(ptr)
    }
  }
}
extension BrowserExtras {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ eval_result: GenericIntoRustString?, _ network_events: RustVec<ResponseMeta>,
    _ cookies: RustVec<CookieInfo>
  ) {
    self.init(
      ptr: __swift_bridge__$BrowserExtras$new(
        {
          if let rustString = optionalStringIntoRustString(eval_result) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          let val = network_events
          val.isOwned = false
          return val.ptr
        }(),
        {
          let val = cookies
          val.isOwned = false
          return val.ptr
        }()))
  }
}
public class BrowserExtrasRefMut: BrowserExtrasRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class BrowserExtrasRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension BrowserExtrasRef {
  public func eval_result() -> RustString? {
    {
      let val = __swift_bridge__$BrowserExtras$eval_result(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func network_events() -> RustVec<ResponseMeta> {
    RustVec(ptr: __swift_bridge__$BrowserExtras$network_events(ptr))
  }

  public func cookies() -> RustVec<CookieInfo> {
    RustVec(ptr: __swift_bridge__$BrowserExtras$cookies(ptr))
  }
}
extension BrowserExtras: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_BrowserExtras$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_BrowserExtras$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BrowserExtras) {
    __swift_bridge__$Vec_BrowserExtras$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_BrowserExtras$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (BrowserExtras(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> BrowserExtrasRef?
  {
    let pointer = __swift_bridge__$Vec_BrowserExtras$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BrowserExtrasRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> BrowserExtrasRefMut?
  {
    let pointer = __swift_bridge__$Vec_BrowserExtras$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BrowserExtrasRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    BrowserExtrasRef
  > {
    UnsafePointer<BrowserExtrasRef>(
      OpaquePointer(__swift_bridge__$Vec_BrowserExtras$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_BrowserExtras$len(vecPtr)
  }
}

public class DownloadedDocument: DownloadedDocumentRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$DownloadedDocument$_free(ptr)
    }
  }
}
extension DownloadedDocument {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ url: GenericIntoRustString, _ mime_type: GenericIntoRustString, _ size: UInt,
    _ filename: GenericIntoRustString?, _ content_hash: GenericIntoRustString,
    _ headers: GenericIntoRustString
  ) {
    self.init(
      ptr: __swift_bridge__$DownloadedDocument$new(
        {
          let rustString = url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let rustString = mime_type.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(), size,
        {
          if let rustString = optionalStringIntoRustString(filename) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          let rustString = content_hash.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let rustString = headers.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }()))
  }
}
public class DownloadedDocumentRefMut: DownloadedDocumentRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class DownloadedDocumentRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension DownloadedDocumentRef {
  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$DownloadedDocument$url(ptr))
  }

  public func mime_type() -> RustString {
    RustString(ptr: __swift_bridge__$DownloadedDocument$mime_type(ptr))
  }

  public func size() -> UInt {
    __swift_bridge__$DownloadedDocument$size(ptr)
  }

  public func filename() -> RustString? {
    {
      let val = __swift_bridge__$DownloadedDocument$filename(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func content_hash() -> RustString {
    RustString(ptr: __swift_bridge__$DownloadedDocument$content_hash(ptr))
  }

  public func headers() -> RustString {
    RustString(ptr: __swift_bridge__$DownloadedDocument$headers(ptr))
  }
}
extension DownloadedDocument: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_DownloadedDocument$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_DownloadedDocument$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: DownloadedDocument) {
    __swift_bridge__$Vec_DownloadedDocument$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_DownloadedDocument$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (DownloadedDocument(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> DownloadedDocumentRef?
  {
    let pointer = __swift_bridge__$Vec_DownloadedDocument$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return DownloadedDocumentRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> DownloadedDocumentRefMut?
  {
    let pointer = __swift_bridge__$Vec_DownloadedDocument$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return DownloadedDocumentRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    DownloadedDocumentRef
  > {
    UnsafePointer<DownloadedDocumentRef>(
      OpaquePointer(__swift_bridge__$Vec_DownloadedDocument$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_DownloadedDocument$len(vecPtr)
  }
}

public class InteractionResult: InteractionResultRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$InteractionResult$_free(ptr)
    }
  }
}
extension InteractionResult {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ action_results: RustVec<ActionResult>, _ final_html: GenericIntoRustString,
    _ final_url: GenericIntoRustString
  ) {
    self.init(
      ptr: __swift_bridge__$InteractionResult$new(
        {
          let val = action_results
          val.isOwned = false
          return val.ptr
        }(),
        {
          let rustString = final_html.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let rustString = final_url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }()))
  }
}
public class InteractionResultRefMut: InteractionResultRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class InteractionResultRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension InteractionResultRef {
  public func action_results() -> RustVec<ActionResult> {
    RustVec(ptr: __swift_bridge__$InteractionResult$action_results(ptr))
  }

  public func final_html() -> RustString {
    RustString(ptr: __swift_bridge__$InteractionResult$final_html(ptr))
  }

  public func final_url() -> RustString {
    RustString(ptr: __swift_bridge__$InteractionResult$final_url(ptr))
  }
}
extension InteractionResult: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_InteractionResult$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_InteractionResult$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: InteractionResult) {
    __swift_bridge__$Vec_InteractionResult$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_InteractionResult$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (InteractionResult(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> InteractionResultRef?
  {
    let pointer = __swift_bridge__$Vec_InteractionResult$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return InteractionResultRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> InteractionResultRefMut?
  {
    let pointer = __swift_bridge__$Vec_InteractionResult$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return InteractionResultRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    InteractionResultRef
  > {
    UnsafePointer<InteractionResultRef>(
      OpaquePointer(__swift_bridge__$Vec_InteractionResult$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_InteractionResult$len(vecPtr)
  }
}

public class ActionResult: ActionResultRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ActionResult$_free(ptr)
    }
  }
}
extension ActionResult {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ action_index: UInt, _ action_type: GenericIntoRustString, _ success: Bool,
    _ data: GenericIntoRustString?, _ error: GenericIntoRustString?
  ) {
    self.init(
      ptr: __swift_bridge__$ActionResult$new(
        action_index,
        {
          let rustString = action_type.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(), success,
        {
          if let rustString = optionalStringIntoRustString(data) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(error) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }()))
  }
}
public class ActionResultRefMut: ActionResultRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ActionResultRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ActionResultRef {
  public func action_index() -> UInt {
    __swift_bridge__$ActionResult$action_index(ptr)
  }

  public func action_type() -> RustString {
    RustString(ptr: __swift_bridge__$ActionResult$action_type(ptr))
  }

  public func success() -> Bool {
    __swift_bridge__$ActionResult$success(ptr)
  }

  public func data() -> RustString? {
    {
      let val = __swift_bridge__$ActionResult$data(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func error() -> RustString? {
    {
      let val = __swift_bridge__$ActionResult$error(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }
}
extension ActionResult: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ActionResult$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ActionResult$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ActionResult) {
    __swift_bridge__$Vec_ActionResult$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_ActionResult$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ActionResult(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> ActionResultRef?
  {
    let pointer = __swift_bridge__$Vec_ActionResult$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ActionResultRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> ActionResultRefMut?
  {
    let pointer = __swift_bridge__$Vec_ActionResult$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ActionResultRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    ActionResultRef
  > {
    UnsafePointer<ActionResultRef>(OpaquePointer(__swift_bridge__$Vec_ActionResult$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ActionResult$len(vecPtr)
  }
}

public class ScrapeResult: ScrapeResultRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ScrapeResult$_free(ptr)
    }
  }
}
extension ScrapeResult {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ status_code: UInt16, _ content_type: GenericIntoRustString, _ html: GenericIntoRustString,
    _ body_size: UInt, _ metadata: PageMetadata, _ links: RustVec<LinkInfo>,
    _ images: RustVec<ImageInfo>, _ feeds: RustVec<FeedInfo>, _ json_ld: RustVec<JsonLdEntry>,
    _ is_allowed: Bool, _ crawl_delay: UInt64?, _ noindex_detected: Bool, _ nofollow_detected: Bool,
    _ x_robots_tag: GenericIntoRustString?, _ is_pdf: Bool, _ was_skipped: Bool,
    _ detected_charset: GenericIntoRustString?, _ auth_header_sent: Bool,
    _ response_meta: ResponseMeta?, _ assets: RustVec<DownloadedAsset>, _ js_render_hint: Bool,
    _ browser_used: Bool, _ markdown: MarkdownResult?, _ extracted_data: GenericIntoRustString?,
    _ extraction_meta: ExtractionMeta?, _ downloaded_document: DownloadedDocument?,
    _ browser: BrowserExtras?
  ) {
    self.init(
      ptr: __swift_bridge__$ScrapeResult$new(
        status_code,
        {
          let rustString = content_type.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let rustString = html.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(), body_size,
        {
          metadata.isOwned = false
          return metadata.ptr
        }(),
        {
          let val = links
          val.isOwned = false
          return val.ptr
        }(),
        {
          let val = images
          val.isOwned = false
          return val.ptr
        }(),
        {
          let val = feeds
          val.isOwned = false
          return val.ptr
        }(),
        {
          let val = json_ld
          val.isOwned = false
          return val.ptr
        }(), is_allowed, crawl_delay.intoFfiRepr(), noindex_detected, nofollow_detected,
        {
          if let rustString = optionalStringIntoRustString(x_robots_tag) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(), is_pdf, was_skipped,
        {
          if let rustString = optionalStringIntoRustString(detected_charset) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(), auth_header_sent,
        {
          if let val = response_meta {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          let val = assets
          val.isOwned = false
          return val.ptr
        }(), js_render_hint, browser_used,
        {
          if let val = markdown {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(extracted_data) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let val = extraction_meta {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          if let val = downloaded_document {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          if let val = browser {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }()))
  }
}
public class ScrapeResultRefMut: ScrapeResultRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ScrapeResultRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ScrapeResultRef {
  public func status_code() -> UInt16 {
    __swift_bridge__$ScrapeResult$status_code(ptr)
  }

  public func content_type() -> RustString {
    RustString(ptr: __swift_bridge__$ScrapeResult$content_type(ptr))
  }

  public func html() -> RustString {
    RustString(ptr: __swift_bridge__$ScrapeResult$html(ptr))
  }

  public func body_size() -> UInt {
    __swift_bridge__$ScrapeResult$body_size(ptr)
  }

  public func metadata() -> PageMetadata {
    PageMetadata(ptr: __swift_bridge__$ScrapeResult$metadata(ptr))
  }

  public func links() -> RustVec<LinkInfo> {
    RustVec(ptr: __swift_bridge__$ScrapeResult$links(ptr))
  }

  public func images() -> RustVec<ImageInfo> {
    RustVec(ptr: __swift_bridge__$ScrapeResult$images(ptr))
  }

  public func feeds() -> RustVec<FeedInfo> {
    RustVec(ptr: __swift_bridge__$ScrapeResult$feeds(ptr))
  }

  public func json_ld() -> RustVec<JsonLdEntry> {
    RustVec(ptr: __swift_bridge__$ScrapeResult$json_ld(ptr))
  }

  public func is_allowed() -> Bool {
    __swift_bridge__$ScrapeResult$is_allowed(ptr)
  }

  public func crawl_delay() -> UInt64? {
    __swift_bridge__$ScrapeResult$crawl_delay(ptr).intoSwiftRepr()
  }

  public func noindex_detected() -> Bool {
    __swift_bridge__$ScrapeResult$noindex_detected(ptr)
  }

  public func nofollow_detected() -> Bool {
    __swift_bridge__$ScrapeResult$nofollow_detected(ptr)
  }

  public func x_robots_tag() -> RustString? {
    {
      let val = __swift_bridge__$ScrapeResult$x_robots_tag(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func is_pdf() -> Bool {
    __swift_bridge__$ScrapeResult$is_pdf(ptr)
  }

  public func was_skipped() -> Bool {
    __swift_bridge__$ScrapeResult$was_skipped(ptr)
  }

  public func detected_charset() -> RustString? {
    {
      let val = __swift_bridge__$ScrapeResult$detected_charset(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func auth_header_sent() -> Bool {
    __swift_bridge__$ScrapeResult$auth_header_sent(ptr)
  }

  public func response_meta() -> ResponseMeta? {
    {
      let val = __swift_bridge__$ScrapeResult$response_meta(ptr)
      if val != nil { return ResponseMeta(ptr: val!) } else { return nil }
    }()
  }

  public func assets() -> RustVec<DownloadedAsset> {
    RustVec(ptr: __swift_bridge__$ScrapeResult$assets(ptr))
  }

  public func js_render_hint() -> Bool {
    __swift_bridge__$ScrapeResult$js_render_hint(ptr)
  }

  public func browser_used() -> Bool {
    __swift_bridge__$ScrapeResult$browser_used(ptr)
  }

  public func markdown() -> MarkdownResult? {
    {
      let val = __swift_bridge__$ScrapeResult$markdown(ptr)
      if val != nil { return MarkdownResult(ptr: val!) } else { return nil }
    }()
  }

  public func extracted_data() -> RustString? {
    {
      let val = __swift_bridge__$ScrapeResult$extracted_data(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func extraction_meta() -> ExtractionMeta? {
    {
      let val = __swift_bridge__$ScrapeResult$extraction_meta(ptr)
      if val != nil { return ExtractionMeta(ptr: val!) } else { return nil }
    }()
  }

  public func downloaded_document() -> DownloadedDocument? {
    {
      let val = __swift_bridge__$ScrapeResult$downloaded_document(ptr)
      if val != nil { return DownloadedDocument(ptr: val!) } else { return nil }
    }()
  }

  public func browser() -> BrowserExtras? {
    {
      let val = __swift_bridge__$ScrapeResult$browser(ptr)
      if val != nil { return BrowserExtras(ptr: val!) } else { return nil }
    }()
  }
}
extension ScrapeResult: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ScrapeResult$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ScrapeResult$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ScrapeResult) {
    __swift_bridge__$Vec_ScrapeResult$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_ScrapeResult$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ScrapeResult(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> ScrapeResultRef?
  {
    let pointer = __swift_bridge__$Vec_ScrapeResult$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ScrapeResultRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> ScrapeResultRefMut?
  {
    let pointer = __swift_bridge__$Vec_ScrapeResult$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ScrapeResultRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    ScrapeResultRef
  > {
    UnsafePointer<ScrapeResultRef>(OpaquePointer(__swift_bridge__$Vec_ScrapeResult$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ScrapeResult$len(vecPtr)
  }
}

public class CrawlPageResult: CrawlPageResultRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$CrawlPageResult$_free(ptr)
    }
  }
}
extension CrawlPageResult {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ url: GenericIntoRustString, _ normalized_url: GenericIntoRustString, _ status_code: UInt16,
    _ content_type: GenericIntoRustString, _ html: GenericIntoRustString, _ body_size: UInt,
    _ metadata: PageMetadata, _ links: RustVec<LinkInfo>, _ images: RustVec<ImageInfo>,
    _ feeds: RustVec<FeedInfo>, _ json_ld: RustVec<JsonLdEntry>, _ depth: UInt,
    _ stayed_on_domain: Bool, _ was_skipped: Bool, _ is_pdf: Bool,
    _ detected_charset: GenericIntoRustString?, _ markdown: MarkdownResult?,
    _ extracted_data: GenericIntoRustString?, _ extraction_meta: ExtractionMeta?,
    _ downloaded_document: DownloadedDocument?
  ) {
    self.init(
      ptr: __swift_bridge__$CrawlPageResult$new(
        {
          let rustString = url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let rustString = normalized_url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(), status_code,
        {
          let rustString = content_type.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let rustString = html.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(), body_size,
        {
          metadata.isOwned = false
          return metadata.ptr
        }(),
        {
          let val = links
          val.isOwned = false
          return val.ptr
        }(),
        {
          let val = images
          val.isOwned = false
          return val.ptr
        }(),
        {
          let val = feeds
          val.isOwned = false
          return val.ptr
        }(),
        {
          let val = json_ld
          val.isOwned = false
          return val.ptr
        }(), depth, stayed_on_domain, was_skipped, is_pdf,
        {
          if let rustString = optionalStringIntoRustString(detected_charset) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let val = markdown {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(extracted_data) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let val = extraction_meta {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          if let val = downloaded_document {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }()))
  }
}
public class CrawlPageResultRefMut: CrawlPageResultRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class CrawlPageResultRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension CrawlPageResultRef {
  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$CrawlPageResult$url(ptr))
  }

  public func normalized_url() -> RustString {
    RustString(ptr: __swift_bridge__$CrawlPageResult$normalized_url(ptr))
  }

  public func status_code() -> UInt16 {
    __swift_bridge__$CrawlPageResult$status_code(ptr)
  }

  public func content_type() -> RustString {
    RustString(ptr: __swift_bridge__$CrawlPageResult$content_type(ptr))
  }

  public func html() -> RustString {
    RustString(ptr: __swift_bridge__$CrawlPageResult$html(ptr))
  }

  public func body_size() -> UInt {
    __swift_bridge__$CrawlPageResult$body_size(ptr)
  }

  public func metadata() -> PageMetadata {
    PageMetadata(ptr: __swift_bridge__$CrawlPageResult$metadata(ptr))
  }

  public func links() -> RustVec<LinkInfo> {
    RustVec(ptr: __swift_bridge__$CrawlPageResult$links(ptr))
  }

  public func images() -> RustVec<ImageInfo> {
    RustVec(ptr: __swift_bridge__$CrawlPageResult$images(ptr))
  }

  public func feeds() -> RustVec<FeedInfo> {
    RustVec(ptr: __swift_bridge__$CrawlPageResult$feeds(ptr))
  }

  public func json_ld() -> RustVec<JsonLdEntry> {
    RustVec(ptr: __swift_bridge__$CrawlPageResult$json_ld(ptr))
  }

  public func depth() -> UInt {
    __swift_bridge__$CrawlPageResult$depth(ptr)
  }

  public func stayed_on_domain() -> Bool {
    __swift_bridge__$CrawlPageResult$stayed_on_domain(ptr)
  }

  public func was_skipped() -> Bool {
    __swift_bridge__$CrawlPageResult$was_skipped(ptr)
  }

  public func is_pdf() -> Bool {
    __swift_bridge__$CrawlPageResult$is_pdf(ptr)
  }

  public func detected_charset() -> RustString? {
    {
      let val = __swift_bridge__$CrawlPageResult$detected_charset(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func markdown() -> MarkdownResult? {
    {
      let val = __swift_bridge__$CrawlPageResult$markdown(ptr)
      if val != nil { return MarkdownResult(ptr: val!) } else { return nil }
    }()
  }

  public func extracted_data() -> RustString? {
    {
      let val = __swift_bridge__$CrawlPageResult$extracted_data(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func extraction_meta() -> ExtractionMeta? {
    {
      let val = __swift_bridge__$CrawlPageResult$extraction_meta(ptr)
      if val != nil { return ExtractionMeta(ptr: val!) } else { return nil }
    }()
  }

  public func downloaded_document() -> DownloadedDocument? {
    {
      let val = __swift_bridge__$CrawlPageResult$downloaded_document(ptr)
      if val != nil { return DownloadedDocument(ptr: val!) } else { return nil }
    }()
  }
}
extension CrawlPageResult: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_CrawlPageResult$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_CrawlPageResult$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CrawlPageResult) {
    __swift_bridge__$Vec_CrawlPageResult$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_CrawlPageResult$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (CrawlPageResult(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> CrawlPageResultRef?
  {
    let pointer = __swift_bridge__$Vec_CrawlPageResult$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CrawlPageResultRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> CrawlPageResultRefMut?
  {
    let pointer = __swift_bridge__$Vec_CrawlPageResult$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CrawlPageResultRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    CrawlPageResultRef
  > {
    UnsafePointer<CrawlPageResultRef>(
      OpaquePointer(__swift_bridge__$Vec_CrawlPageResult$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_CrawlPageResult$len(vecPtr)
  }
}

public class CrawlResult: CrawlResultRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$CrawlResult$_free(ptr)
    }
  }
}
extension CrawlResult {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ pages: RustVec<CrawlPageResult>, _ final_url: GenericIntoRustString, _ redirect_count: UInt,
    _ was_skipped: Bool, _ error: GenericIntoRustString?, _ cookies: RustVec<CookieInfo>
  ) {
    self.init(
      ptr: __swift_bridge__$CrawlResult$new(
        {
          let val = pages
          val.isOwned = false
          return val.ptr
        }(),
        {
          let rustString = final_url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(), redirect_count, was_skipped,
        {
          if let rustString = optionalStringIntoRustString(error) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          let val = cookies
          val.isOwned = false
          return val.ptr
        }()))
  }
}
public class CrawlResultRefMut: CrawlResultRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class CrawlResultRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension CrawlResultRef {
  public func pages() -> RustVec<CrawlPageResult> {
    RustVec(ptr: __swift_bridge__$CrawlResult$pages(ptr))
  }

  public func final_url() -> RustString {
    RustString(ptr: __swift_bridge__$CrawlResult$final_url(ptr))
  }

  public func redirect_count() -> UInt {
    __swift_bridge__$CrawlResult$redirect_count(ptr)
  }

  public func was_skipped() -> Bool {
    __swift_bridge__$CrawlResult$was_skipped(ptr)
  }

  public func error() -> RustString? {
    {
      let val = __swift_bridge__$CrawlResult$error(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func cookies() -> RustVec<CookieInfo> {
    RustVec(ptr: __swift_bridge__$CrawlResult$cookies(ptr))
  }
}
extension CrawlResult: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_CrawlResult$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_CrawlResult$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CrawlResult) {
    __swift_bridge__$Vec_CrawlResult$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_CrawlResult$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (CrawlResult(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> CrawlResultRef? {
    let pointer = __swift_bridge__$Vec_CrawlResult$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CrawlResultRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> CrawlResultRefMut?
  {
    let pointer = __swift_bridge__$Vec_CrawlResult$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CrawlResultRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    CrawlResultRef
  > {
    UnsafePointer<CrawlResultRef>(OpaquePointer(__swift_bridge__$Vec_CrawlResult$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_CrawlResult$len(vecPtr)
  }
}

public class SitemapUrl: SitemapUrlRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$SitemapUrl$_free(ptr)
    }
  }
}
extension SitemapUrl {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ url: GenericIntoRustString, _ lastmod: GenericIntoRustString?,
    _ changefreq: GenericIntoRustString?, _ priority: GenericIntoRustString?
  ) {
    self.init(
      ptr: __swift_bridge__$SitemapUrl$new(
        {
          let rustString = url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(lastmod) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(changefreq) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(priority) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }()))
  }
}
public class SitemapUrlRefMut: SitemapUrlRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class SitemapUrlRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension SitemapUrlRef {
  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$SitemapUrl$url(ptr))
  }

  public func lastmod() -> RustString? {
    {
      let val = __swift_bridge__$SitemapUrl$lastmod(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func changefreq() -> RustString? {
    {
      let val = __swift_bridge__$SitemapUrl$changefreq(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func priority() -> RustString? {
    {
      let val = __swift_bridge__$SitemapUrl$priority(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }
}
extension SitemapUrl: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_SitemapUrl$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_SitemapUrl$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: SitemapUrl) {
    __swift_bridge__$Vec_SitemapUrl$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_SitemapUrl$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (SitemapUrl(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> SitemapUrlRef? {
    let pointer = __swift_bridge__$Vec_SitemapUrl$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return SitemapUrlRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> SitemapUrlRefMut?
  {
    let pointer = __swift_bridge__$Vec_SitemapUrl$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return SitemapUrlRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SitemapUrlRef>
  {
    UnsafePointer<SitemapUrlRef>(OpaquePointer(__swift_bridge__$Vec_SitemapUrl$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_SitemapUrl$len(vecPtr)
  }
}

public class MapResult: MapResultRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$MapResult$_free(ptr)
    }
  }
}
extension MapResult {
  public convenience init(_ urls: RustVec<SitemapUrl>) {
    self.init(
      ptr: __swift_bridge__$MapResult$new(
        {
          let val = urls
          val.isOwned = false
          return val.ptr
        }()))
  }
}
public class MapResultRefMut: MapResultRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class MapResultRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension MapResultRef {
  public func urls() -> RustVec<SitemapUrl> {
    RustVec(ptr: __swift_bridge__$MapResult$urls(ptr))
  }
}
extension MapResult: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_MapResult$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_MapResult$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: MapResult) {
    __swift_bridge__$Vec_MapResult$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_MapResult$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (MapResult(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> MapResultRef? {
    let pointer = __swift_bridge__$Vec_MapResult$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return MapResultRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> MapResultRefMut?
  {
    let pointer = __swift_bridge__$Vec_MapResult$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return MapResultRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<MapResultRef>
  {
    UnsafePointer<MapResultRef>(OpaquePointer(__swift_bridge__$Vec_MapResult$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_MapResult$len(vecPtr)
  }
}

public class MarkdownResult: MarkdownResultRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$MarkdownResult$_free(ptr)
    }
  }
}
extension MarkdownResult {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ content: GenericIntoRustString, _ document_structure: GenericIntoRustString?,
    _ tables: RustVec<GenericIntoRustString>, _ warnings: RustVec<GenericIntoRustString>,
    _ citations: CitationResult?, _ fit_content: GenericIntoRustString?
  ) {
    self.init(
      ptr: __swift_bridge__$MarkdownResult$new(
        {
          let rustString = content.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(document_structure) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          let val = tables
          val.isOwned = false
          return val.ptr
        }(),
        {
          let val = warnings
          val.isOwned = false
          return val.ptr
        }(),
        {
          if let val = citations {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(fit_content) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }()))
  }
}
public class MarkdownResultRefMut: MarkdownResultRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class MarkdownResultRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension MarkdownResultRef {
  public func content() -> RustString {
    RustString(ptr: __swift_bridge__$MarkdownResult$content(ptr))
  }

  public func document_structure() -> RustString? {
    {
      let val = __swift_bridge__$MarkdownResult$document_structure(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func tables() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$MarkdownResult$tables(ptr))
  }

  public func warnings() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$MarkdownResult$warnings(ptr))
  }

  public func citations() -> CitationResult? {
    {
      let val = __swift_bridge__$MarkdownResult$citations(ptr)
      if val != nil { return CitationResult(ptr: val!) } else { return nil }
    }()
  }

  public func fit_content() -> RustString? {
    {
      let val = __swift_bridge__$MarkdownResult$fit_content(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }
}
extension MarkdownResult: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_MarkdownResult$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_MarkdownResult$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: MarkdownResult) {
    __swift_bridge__$Vec_MarkdownResult$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_MarkdownResult$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (MarkdownResult(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> MarkdownResultRef?
  {
    let pointer = __swift_bridge__$Vec_MarkdownResult$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return MarkdownResultRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> MarkdownResultRefMut?
  {
    let pointer = __swift_bridge__$Vec_MarkdownResult$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return MarkdownResultRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    MarkdownResultRef
  > {
    UnsafePointer<MarkdownResultRef>(
      OpaquePointer(__swift_bridge__$Vec_MarkdownResult$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_MarkdownResult$len(vecPtr)
  }
}

public class LinkInfo: LinkInfoRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$LinkInfo$_free(ptr)
    }
  }
}
extension LinkInfo {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ url: GenericIntoRustString, _ text: GenericIntoRustString, _ link_type: LinkType,
    _ rel: GenericIntoRustString?, _ nofollow: Bool
  ) {
    self.init(
      ptr: __swift_bridge__$LinkInfo$new(
        {
          let rustString = url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let rustString = text.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          link_type.isOwned = false
          return link_type.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(rel) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(), nofollow))
  }
}
public class LinkInfoRefMut: LinkInfoRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class LinkInfoRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension LinkInfoRef {
  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$LinkInfo$url(ptr))
  }

  public func text() -> RustString {
    RustString(ptr: __swift_bridge__$LinkInfo$text(ptr))
  }

  public func link_type() -> RustString {
    RustString(ptr: __swift_bridge__$LinkInfo$link_type(ptr))
  }

  public func rel() -> RustString? {
    {
      let val = __swift_bridge__$LinkInfo$rel(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func nofollow() -> Bool {
    __swift_bridge__$LinkInfo$nofollow(ptr)
  }
}
extension LinkInfo: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_LinkInfo$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_LinkInfo$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: LinkInfo) {
    __swift_bridge__$Vec_LinkInfo$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_LinkInfo$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (LinkInfo(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> LinkInfoRef? {
    let pointer = __swift_bridge__$Vec_LinkInfo$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return LinkInfoRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> LinkInfoRefMut?
  {
    let pointer = __swift_bridge__$Vec_LinkInfo$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return LinkInfoRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<LinkInfoRef> {
    UnsafePointer<LinkInfoRef>(OpaquePointer(__swift_bridge__$Vec_LinkInfo$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_LinkInfo$len(vecPtr)
  }
}

public class ImageInfo: ImageInfoRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ImageInfo$_free(ptr)
    }
  }
}
extension ImageInfo {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ url: GenericIntoRustString, _ alt: GenericIntoRustString?, _ width: UInt32?,
    _ height: UInt32?, _ source: ImageSource
  ) {
    self.init(
      ptr: __swift_bridge__$ImageInfo$new(
        {
          let rustString = url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(alt) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(), width.intoFfiRepr(), height.intoFfiRepr(),
        {
          source.isOwned = false
          return source.ptr
        }()))
  }
}
public class ImageInfoRefMut: ImageInfoRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ImageInfoRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ImageInfoRef {
  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$ImageInfo$url(ptr))
  }

  public func alt() -> RustString? {
    {
      let val = __swift_bridge__$ImageInfo$alt(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func width() -> UInt32? {
    __swift_bridge__$ImageInfo$width(ptr).intoSwiftRepr()
  }

  public func height() -> UInt32? {
    __swift_bridge__$ImageInfo$height(ptr).intoSwiftRepr()
  }

  public func source() -> RustString {
    RustString(ptr: __swift_bridge__$ImageInfo$source(ptr))
  }
}
extension ImageInfo: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ImageInfo$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ImageInfo$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ImageInfo) {
    __swift_bridge__$Vec_ImageInfo$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_ImageInfo$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ImageInfo(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> ImageInfoRef? {
    let pointer = __swift_bridge__$Vec_ImageInfo$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ImageInfoRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> ImageInfoRefMut?
  {
    let pointer = __swift_bridge__$Vec_ImageInfo$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ImageInfoRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ImageInfoRef>
  {
    UnsafePointer<ImageInfoRef>(OpaquePointer(__swift_bridge__$Vec_ImageInfo$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ImageInfo$len(vecPtr)
  }
}

public class FeedInfo: FeedInfoRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$FeedInfo$_free(ptr)
    }
  }
}
extension FeedInfo {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ url: GenericIntoRustString, _ title: GenericIntoRustString?, _ feed_type: FeedType
  ) {
    self.init(
      ptr: __swift_bridge__$FeedInfo$new(
        {
          let rustString = url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(title) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          feed_type.isOwned = false
          return feed_type.ptr
        }()))
  }
}
public class FeedInfoRefMut: FeedInfoRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class FeedInfoRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension FeedInfoRef {
  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$FeedInfo$url(ptr))
  }

  public func title() -> RustString? {
    {
      let val = __swift_bridge__$FeedInfo$title(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func feed_type() -> RustString {
    RustString(ptr: __swift_bridge__$FeedInfo$feed_type(ptr))
  }
}
extension FeedInfo: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_FeedInfo$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_FeedInfo$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: FeedInfo) {
    __swift_bridge__$Vec_FeedInfo$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_FeedInfo$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (FeedInfo(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> FeedInfoRef? {
    let pointer = __swift_bridge__$Vec_FeedInfo$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return FeedInfoRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> FeedInfoRefMut?
  {
    let pointer = __swift_bridge__$Vec_FeedInfo$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return FeedInfoRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<FeedInfoRef> {
    UnsafePointer<FeedInfoRef>(OpaquePointer(__swift_bridge__$Vec_FeedInfo$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_FeedInfo$len(vecPtr)
  }
}

public class JsonLdEntry: JsonLdEntryRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$JsonLdEntry$_free(ptr)
    }
  }
}
extension JsonLdEntry {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ schema_type: GenericIntoRustString, _ name: GenericIntoRustString?,
    _ raw: GenericIntoRustString
  ) {
    self.init(
      ptr: __swift_bridge__$JsonLdEntry$new(
        {
          let rustString = schema_type.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(name) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          let rustString = raw.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }()))
  }
}
public class JsonLdEntryRefMut: JsonLdEntryRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class JsonLdEntryRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension JsonLdEntryRef {
  public func schema_type() -> RustString {
    RustString(ptr: __swift_bridge__$JsonLdEntry$schema_type(ptr))
  }

  public func name() -> RustString? {
    {
      let val = __swift_bridge__$JsonLdEntry$name(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func raw() -> RustString {
    RustString(ptr: __swift_bridge__$JsonLdEntry$raw(ptr))
  }
}
extension JsonLdEntry: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_JsonLdEntry$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_JsonLdEntry$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: JsonLdEntry) {
    __swift_bridge__$Vec_JsonLdEntry$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_JsonLdEntry$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (JsonLdEntry(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> JsonLdEntryRef? {
    let pointer = __swift_bridge__$Vec_JsonLdEntry$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return JsonLdEntryRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> JsonLdEntryRefMut?
  {
    let pointer = __swift_bridge__$Vec_JsonLdEntry$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return JsonLdEntryRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    JsonLdEntryRef
  > {
    UnsafePointer<JsonLdEntryRef>(OpaquePointer(__swift_bridge__$Vec_JsonLdEntry$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_JsonLdEntry$len(vecPtr)
  }
}

public class CookieInfo: CookieInfoRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$CookieInfo$_free(ptr)
    }
  }
}
extension CookieInfo {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ name: GenericIntoRustString, _ value: GenericIntoRustString, _ domain: GenericIntoRustString?,
    _ path: GenericIntoRustString?
  ) {
    self.init(
      ptr: __swift_bridge__$CookieInfo$new(
        {
          let rustString = name.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let rustString = value.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(domain) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(path) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }()))
  }
}
public class CookieInfoRefMut: CookieInfoRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class CookieInfoRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension CookieInfoRef {
  public func name() -> RustString {
    RustString(ptr: __swift_bridge__$CookieInfo$name(ptr))
  }

  public func value() -> RustString {
    RustString(ptr: __swift_bridge__$CookieInfo$value(ptr))
  }

  public func domain() -> RustString? {
    {
      let val = __swift_bridge__$CookieInfo$domain(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func path() -> RustString? {
    {
      let val = __swift_bridge__$CookieInfo$path(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }
}
extension CookieInfo: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_CookieInfo$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_CookieInfo$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CookieInfo) {
    __swift_bridge__$Vec_CookieInfo$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_CookieInfo$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (CookieInfo(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> CookieInfoRef? {
    let pointer = __swift_bridge__$Vec_CookieInfo$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CookieInfoRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> CookieInfoRefMut?
  {
    let pointer = __swift_bridge__$Vec_CookieInfo$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CookieInfoRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CookieInfoRef>
  {
    UnsafePointer<CookieInfoRef>(OpaquePointer(__swift_bridge__$Vec_CookieInfo$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_CookieInfo$len(vecPtr)
  }
}

public class DownloadedAsset: DownloadedAssetRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$DownloadedAsset$_free(ptr)
    }
  }
}
extension DownloadedAsset {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ url: GenericIntoRustString, _ content_hash: GenericIntoRustString,
    _ mime_type: GenericIntoRustString?, _ size: UInt, _ asset_category: AssetCategory,
    _ html_tag: GenericIntoRustString?
  ) {
    self.init(
      ptr: __swift_bridge__$DownloadedAsset$new(
        {
          let rustString = url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let rustString = content_hash.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(mime_type) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(), size,
        {
          asset_category.isOwned = false
          return asset_category.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(html_tag) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }()))
  }
}
public class DownloadedAssetRefMut: DownloadedAssetRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class DownloadedAssetRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension DownloadedAssetRef {
  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$DownloadedAsset$url(ptr))
  }

  public func content_hash() -> RustString {
    RustString(ptr: __swift_bridge__$DownloadedAsset$content_hash(ptr))
  }

  public func mime_type() -> RustString? {
    {
      let val = __swift_bridge__$DownloadedAsset$mime_type(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func size() -> UInt {
    __swift_bridge__$DownloadedAsset$size(ptr)
  }

  public func asset_category() -> RustString {
    RustString(ptr: __swift_bridge__$DownloadedAsset$asset_category(ptr))
  }

  public func html_tag() -> RustString? {
    {
      let val = __swift_bridge__$DownloadedAsset$html_tag(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }
}
extension DownloadedAsset: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_DownloadedAsset$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_DownloadedAsset$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: DownloadedAsset) {
    __swift_bridge__$Vec_DownloadedAsset$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_DownloadedAsset$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (DownloadedAsset(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> DownloadedAssetRef?
  {
    let pointer = __swift_bridge__$Vec_DownloadedAsset$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return DownloadedAssetRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> DownloadedAssetRefMut?
  {
    let pointer = __swift_bridge__$Vec_DownloadedAsset$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return DownloadedAssetRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    DownloadedAssetRef
  > {
    UnsafePointer<DownloadedAssetRef>(
      OpaquePointer(__swift_bridge__$Vec_DownloadedAsset$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_DownloadedAsset$len(vecPtr)
  }
}

public class ArticleMetadata: ArticleMetadataRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ArticleMetadata$_free(ptr)
    }
  }
}
extension ArticleMetadata {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ published_time: GenericIntoRustString?, _ modified_time: GenericIntoRustString?,
    _ author: GenericIntoRustString?, _ section: GenericIntoRustString?,
    _ tags: RustVec<GenericIntoRustString>
  ) {
    self.init(
      ptr: __swift_bridge__$ArticleMetadata$new(
        {
          if let rustString = optionalStringIntoRustString(published_time) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(modified_time) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(author) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(section) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          let val = tags
          val.isOwned = false
          return val.ptr
        }()))
  }
}
public class ArticleMetadataRefMut: ArticleMetadataRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ArticleMetadataRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ArticleMetadataRef {
  public func published_time() -> RustString? {
    {
      let val = __swift_bridge__$ArticleMetadata$published_time(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func modified_time() -> RustString? {
    {
      let val = __swift_bridge__$ArticleMetadata$modified_time(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func author() -> RustString? {
    {
      let val = __swift_bridge__$ArticleMetadata$author(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func section() -> RustString? {
    {
      let val = __swift_bridge__$ArticleMetadata$section(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func tags() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$ArticleMetadata$tags(ptr))
  }
}
extension ArticleMetadata: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ArticleMetadata$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ArticleMetadata$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ArticleMetadata) {
    __swift_bridge__$Vec_ArticleMetadata$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_ArticleMetadata$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ArticleMetadata(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> ArticleMetadataRef?
  {
    let pointer = __swift_bridge__$Vec_ArticleMetadata$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ArticleMetadataRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> ArticleMetadataRefMut?
  {
    let pointer = __swift_bridge__$Vec_ArticleMetadata$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ArticleMetadataRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    ArticleMetadataRef
  > {
    UnsafePointer<ArticleMetadataRef>(
      OpaquePointer(__swift_bridge__$Vec_ArticleMetadata$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ArticleMetadata$len(vecPtr)
  }
}

public class HreflangEntry: HreflangEntryRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$HreflangEntry$_free(ptr)
    }
  }
}
extension HreflangEntry {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ lang: GenericIntoRustString, _ url: GenericIntoRustString
  ) {
    self.init(
      ptr: __swift_bridge__$HreflangEntry$new(
        {
          let rustString = lang.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let rustString = url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }()))
  }
}
public class HreflangEntryRefMut: HreflangEntryRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class HreflangEntryRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension HreflangEntryRef {
  public func lang() -> RustString {
    RustString(ptr: __swift_bridge__$HreflangEntry$lang(ptr))
  }

  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$HreflangEntry$url(ptr))
  }
}
extension HreflangEntry: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_HreflangEntry$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_HreflangEntry$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: HreflangEntry) {
    __swift_bridge__$Vec_HreflangEntry$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_HreflangEntry$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (HreflangEntry(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> HreflangEntryRef?
  {
    let pointer = __swift_bridge__$Vec_HreflangEntry$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return HreflangEntryRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> HreflangEntryRefMut?
  {
    let pointer = __swift_bridge__$Vec_HreflangEntry$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return HreflangEntryRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    HreflangEntryRef
  > {
    UnsafePointer<HreflangEntryRef>(
      OpaquePointer(__swift_bridge__$Vec_HreflangEntry$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_HreflangEntry$len(vecPtr)
  }
}

public class FaviconInfo: FaviconInfoRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$FaviconInfo$_free(ptr)
    }
  }
}
extension FaviconInfo {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ url: GenericIntoRustString, _ rel: GenericIntoRustString, _ sizes: GenericIntoRustString?,
    _ mime_type: GenericIntoRustString?
  ) {
    self.init(
      ptr: __swift_bridge__$FaviconInfo$new(
        {
          let rustString = url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let rustString = rel.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          if let rustString = optionalStringIntoRustString(sizes) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(mime_type) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }()))
  }
}
public class FaviconInfoRefMut: FaviconInfoRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class FaviconInfoRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension FaviconInfoRef {
  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$FaviconInfo$url(ptr))
  }

  public func rel() -> RustString {
    RustString(ptr: __swift_bridge__$FaviconInfo$rel(ptr))
  }

  public func sizes() -> RustString? {
    {
      let val = __swift_bridge__$FaviconInfo$sizes(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func mime_type() -> RustString? {
    {
      let val = __swift_bridge__$FaviconInfo$mime_type(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }
}
extension FaviconInfo: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_FaviconInfo$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_FaviconInfo$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: FaviconInfo) {
    __swift_bridge__$Vec_FaviconInfo$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_FaviconInfo$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (FaviconInfo(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> FaviconInfoRef? {
    let pointer = __swift_bridge__$Vec_FaviconInfo$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return FaviconInfoRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> FaviconInfoRefMut?
  {
    let pointer = __swift_bridge__$Vec_FaviconInfo$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return FaviconInfoRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    FaviconInfoRef
  > {
    UnsafePointer<FaviconInfoRef>(OpaquePointer(__swift_bridge__$Vec_FaviconInfo$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_FaviconInfo$len(vecPtr)
  }
}

public class HeadingInfo: HeadingInfoRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$HeadingInfo$_free(ptr)
    }
  }
}
extension HeadingInfo {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ level: UInt8, _ text: GenericIntoRustString
  ) {
    self.init(
      ptr: __swift_bridge__$HeadingInfo$new(
        level,
        {
          let rustString = text.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }()))
  }
}
public class HeadingInfoRefMut: HeadingInfoRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class HeadingInfoRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension HeadingInfoRef {
  public func level() -> UInt8 {
    __swift_bridge__$HeadingInfo$level(ptr)
  }

  public func text() -> RustString {
    RustString(ptr: __swift_bridge__$HeadingInfo$text(ptr))
  }
}
extension HeadingInfo: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_HeadingInfo$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_HeadingInfo$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: HeadingInfo) {
    __swift_bridge__$Vec_HeadingInfo$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_HeadingInfo$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (HeadingInfo(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> HeadingInfoRef? {
    let pointer = __swift_bridge__$Vec_HeadingInfo$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return HeadingInfoRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> HeadingInfoRefMut?
  {
    let pointer = __swift_bridge__$Vec_HeadingInfo$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return HeadingInfoRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    HeadingInfoRef
  > {
    UnsafePointer<HeadingInfoRef>(OpaquePointer(__swift_bridge__$Vec_HeadingInfo$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_HeadingInfo$len(vecPtr)
  }
}

public class ResponseMeta: ResponseMetaRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ResponseMeta$_free(ptr)
    }
  }
}
extension ResponseMeta {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ etag: GenericIntoRustString?, _ last_modified: GenericIntoRustString?,
    _ cache_control: GenericIntoRustString?, _ server: GenericIntoRustString?,
    _ x_powered_by: GenericIntoRustString?, _ content_language: GenericIntoRustString?,
    _ content_encoding: GenericIntoRustString?
  ) {
    self.init(
      ptr: __swift_bridge__$ResponseMeta$new(
        {
          if let rustString = optionalStringIntoRustString(etag) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(last_modified) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(cache_control) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(server) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(x_powered_by) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(content_language) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(content_encoding) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }()))
  }
}
public class ResponseMetaRefMut: ResponseMetaRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ResponseMetaRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ResponseMetaRef {
  public func etag() -> RustString? {
    {
      let val = __swift_bridge__$ResponseMeta$etag(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func last_modified() -> RustString? {
    {
      let val = __swift_bridge__$ResponseMeta$last_modified(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func cache_control() -> RustString? {
    {
      let val = __swift_bridge__$ResponseMeta$cache_control(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func server() -> RustString? {
    {
      let val = __swift_bridge__$ResponseMeta$server(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func x_powered_by() -> RustString? {
    {
      let val = __swift_bridge__$ResponseMeta$x_powered_by(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func content_language() -> RustString? {
    {
      let val = __swift_bridge__$ResponseMeta$content_language(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func content_encoding() -> RustString? {
    {
      let val = __swift_bridge__$ResponseMeta$content_encoding(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }
}
extension ResponseMeta: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ResponseMeta$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ResponseMeta$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ResponseMeta) {
    __swift_bridge__$Vec_ResponseMeta$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_ResponseMeta$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ResponseMeta(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> ResponseMetaRef?
  {
    let pointer = __swift_bridge__$Vec_ResponseMeta$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ResponseMetaRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> ResponseMetaRefMut?
  {
    let pointer = __swift_bridge__$Vec_ResponseMeta$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ResponseMetaRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    ResponseMetaRef
  > {
    UnsafePointer<ResponseMetaRef>(OpaquePointer(__swift_bridge__$Vec_ResponseMeta$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ResponseMeta$len(vecPtr)
  }
}

public class PageMetadata: PageMetadataRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$PageMetadata$_free(ptr)
    }
  }
}
extension PageMetadata {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ title: GenericIntoRustString?, _ description: GenericIntoRustString?,
    _ canonical_url: GenericIntoRustString?, _ keywords: GenericIntoRustString?,
    _ author: GenericIntoRustString?, _ viewport: GenericIntoRustString?,
    _ theme_color: GenericIntoRustString?, _ generator: GenericIntoRustString?,
    _ robots: GenericIntoRustString?, _ html_lang: GenericIntoRustString?,
    _ html_dir: GenericIntoRustString?, _ og_title: GenericIntoRustString?,
    _ og_type: GenericIntoRustString?, _ og_image: GenericIntoRustString?,
    _ og_description: GenericIntoRustString?, _ og_url: GenericIntoRustString?,
    _ og_site_name: GenericIntoRustString?, _ og_locale: GenericIntoRustString?,
    _ og_video: GenericIntoRustString?, _ og_audio: GenericIntoRustString?,
    _ og_locale_alternates: RustVec<GenericIntoRustString>?, _ twitter_card: GenericIntoRustString?,
    _ twitter_title: GenericIntoRustString?, _ twitter_description: GenericIntoRustString?,
    _ twitter_image: GenericIntoRustString?, _ twitter_site: GenericIntoRustString?,
    _ twitter_creator: GenericIntoRustString?, _ dc_title: GenericIntoRustString?,
    _ dc_creator: GenericIntoRustString?, _ dc_subject: GenericIntoRustString?,
    _ dc_description: GenericIntoRustString?, _ dc_publisher: GenericIntoRustString?,
    _ dc_date: GenericIntoRustString?, _ dc_type: GenericIntoRustString?,
    _ dc_format: GenericIntoRustString?, _ dc_identifier: GenericIntoRustString?,
    _ dc_language: GenericIntoRustString?, _ dc_rights: GenericIntoRustString?,
    _ article: ArticleMetadata?, _ hreflangs: RustVec<HreflangEntry>?,
    _ favicons: RustVec<FaviconInfo>?, _ headings: RustVec<HeadingInfo>?, _ word_count: UInt?
  ) {
    self.init(
      ptr: __swift_bridge__$PageMetadata$new(
        {
          if let rustString = optionalStringIntoRustString(title) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(description) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(canonical_url) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(keywords) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(author) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(viewport) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(theme_color) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(generator) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(robots) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(html_lang) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(html_dir) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(og_title) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(og_type) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(og_image) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(og_description) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(og_url) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(og_site_name) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(og_locale) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(og_video) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(og_audio) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let val = og_locale_alternates {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(twitter_card) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(twitter_title) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(twitter_description) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(twitter_image) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(twitter_site) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(twitter_creator) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(dc_title) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(dc_creator) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(dc_subject) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(dc_description) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(dc_publisher) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(dc_date) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(dc_type) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(dc_format) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(dc_identifier) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(dc_language) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(dc_rights) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }(),
        {
          if let val = article {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          if let val = hreflangs {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          if let val = favicons {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          if let val = headings {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(), word_count.intoFfiRepr()))
  }
}
public class PageMetadataRefMut: PageMetadataRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class PageMetadataRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension PageMetadataRef {
  public func title() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$title(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func description() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$description(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func canonical_url() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$canonical_url(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func keywords() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$keywords(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func author() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$author(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func viewport() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$viewport(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func theme_color() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$theme_color(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func generator() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$generator(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func robots() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$robots(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func html_lang() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$html_lang(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func html_dir() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$html_dir(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func og_title() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$og_title(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func og_type() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$og_type(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func og_image() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$og_image(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func og_description() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$og_description(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func og_url() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$og_url(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func og_site_name() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$og_site_name(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func og_locale() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$og_locale(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func og_video() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$og_video(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func og_audio() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$og_audio(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func og_locale_alternates() -> RustVec<RustString>? {
    {
      let val = __swift_bridge__$PageMetadata$og_locale_alternates(ptr)
      if val != nil { return RustVec(ptr: val!) } else { return nil }
    }()
  }

  public func twitter_card() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$twitter_card(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func twitter_title() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$twitter_title(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func twitter_description() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$twitter_description(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func twitter_image() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$twitter_image(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func twitter_site() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$twitter_site(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func twitter_creator() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$twitter_creator(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func dc_title() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$dc_title(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func dc_creator() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$dc_creator(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func dc_subject() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$dc_subject(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func dc_description() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$dc_description(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func dc_publisher() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$dc_publisher(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func dc_date() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$dc_date(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func dc_type() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$dc_type(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func dc_format() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$dc_format(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func dc_identifier() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$dc_identifier(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func dc_language() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$dc_language(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func dc_rights() -> RustString? {
    {
      let val = __swift_bridge__$PageMetadata$dc_rights(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }

  public func article() -> ArticleMetadata? {
    {
      let val = __swift_bridge__$PageMetadata$article(ptr)
      if val != nil { return ArticleMetadata(ptr: val!) } else { return nil }
    }()
  }

  public func hreflangs() -> RustVec<HreflangEntry>? {
    {
      let val = __swift_bridge__$PageMetadata$hreflangs(ptr)
      if val != nil { return RustVec(ptr: val!) } else { return nil }
    }()
  }

  public func favicons() -> RustVec<FaviconInfo>? {
    {
      let val = __swift_bridge__$PageMetadata$favicons(ptr)
      if val != nil { return RustVec(ptr: val!) } else { return nil }
    }()
  }

  public func headings() -> RustVec<HeadingInfo>? {
    {
      let val = __swift_bridge__$PageMetadata$headings(ptr)
      if val != nil { return RustVec(ptr: val!) } else { return nil }
    }()
  }

  public func word_count() -> UInt? {
    __swift_bridge__$PageMetadata$word_count(ptr).intoSwiftRepr()
  }
}
extension PageMetadata: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_PageMetadata$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_PageMetadata$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: PageMetadata) {
    __swift_bridge__$Vec_PageMetadata$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_PageMetadata$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (PageMetadata(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> PageMetadataRef?
  {
    let pointer = __swift_bridge__$Vec_PageMetadata$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return PageMetadataRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> PageMetadataRefMut?
  {
    let pointer = __swift_bridge__$Vec_PageMetadata$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return PageMetadataRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    PageMetadataRef
  > {
    UnsafePointer<PageMetadataRef>(OpaquePointer(__swift_bridge__$Vec_PageMetadata$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_PageMetadata$len(vecPtr)
  }
}

public class CitationResult: CitationResultRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$CitationResult$_free(ptr)
    }
  }
}
extension CitationResult {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ content: GenericIntoRustString, _ references: RustVec<CitationReference>
  ) {
    self.init(
      ptr: __swift_bridge__$CitationResult$new(
        {
          let rustString = content.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let val = references
          val.isOwned = false
          return val.ptr
        }()))
  }
}
public class CitationResultRefMut: CitationResultRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class CitationResultRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension CitationResultRef {
  public func content() -> RustString {
    RustString(ptr: __swift_bridge__$CitationResult$content(ptr))
  }

  public func references() -> RustVec<CitationReference> {
    RustVec(ptr: __swift_bridge__$CitationResult$references(ptr))
  }
}
extension CitationResult: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_CitationResult$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_CitationResult$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CitationResult) {
    __swift_bridge__$Vec_CitationResult$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_CitationResult$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (CitationResult(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> CitationResultRef?
  {
    let pointer = __swift_bridge__$Vec_CitationResult$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CitationResultRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> CitationResultRefMut?
  {
    let pointer = __swift_bridge__$Vec_CitationResult$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CitationResultRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    CitationResultRef
  > {
    UnsafePointer<CitationResultRef>(
      OpaquePointer(__swift_bridge__$Vec_CitationResult$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_CitationResult$len(vecPtr)
  }
}

public class CitationReference: CitationReferenceRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$CitationReference$_free(ptr)
    }
  }
}
extension CitationReference {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ index: UInt, _ url: GenericIntoRustString, _ text: GenericIntoRustString
  ) {
    self.init(
      ptr: __swift_bridge__$CitationReference$new(
        index,
        {
          let rustString = url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          let rustString = text.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }()))
  }
}
public class CitationReferenceRefMut: CitationReferenceRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class CitationReferenceRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension CitationReferenceRef {
  public func index() -> UInt {
    __swift_bridge__$CitationReference$index(ptr)
  }

  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$CitationReference$url(ptr))
  }

  public func text() -> RustString {
    RustString(ptr: __swift_bridge__$CitationReference$text(ptr))
  }
}
extension CitationReference: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_CitationReference$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_CitationReference$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CitationReference) {
    __swift_bridge__$Vec_CitationReference$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_CitationReference$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (CitationReference(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> CitationReferenceRef?
  {
    let pointer = __swift_bridge__$Vec_CitationReference$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CitationReferenceRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> CitationReferenceRefMut?
  {
    let pointer = __swift_bridge__$Vec_CitationReference$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CitationReferenceRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    CitationReferenceRef
  > {
    UnsafePointer<CitationReferenceRef>(
      OpaquePointer(__swift_bridge__$Vec_CitationReference$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_CitationReference$len(vecPtr)
  }
}

public class CrawlEngineHandle: CrawlEngineHandleRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$CrawlEngineHandle$_free(ptr)
    }
  }
}
public class CrawlEngineHandleRefMut: CrawlEngineHandleRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class CrawlEngineHandleRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension CrawlEngineHandle: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_CrawlEngineHandle$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_CrawlEngineHandle$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CrawlEngineHandle) {
    __swift_bridge__$Vec_CrawlEngineHandle$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_CrawlEngineHandle$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (CrawlEngineHandle(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> CrawlEngineHandleRef?
  {
    let pointer = __swift_bridge__$Vec_CrawlEngineHandle$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CrawlEngineHandleRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> CrawlEngineHandleRefMut?
  {
    let pointer = __swift_bridge__$Vec_CrawlEngineHandle$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CrawlEngineHandleRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    CrawlEngineHandleRef
  > {
    UnsafePointer<CrawlEngineHandleRef>(
      OpaquePointer(__swift_bridge__$Vec_CrawlEngineHandle$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_CrawlEngineHandle$len(vecPtr)
  }
}

public class BatchScrapeResult: BatchScrapeResultRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$BatchScrapeResult$_free(ptr)
    }
  }
}
extension BatchScrapeResult {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ url: GenericIntoRustString, _ result: ScrapeResult?, _ error: GenericIntoRustString?
  ) {
    self.init(
      ptr: __swift_bridge__$BatchScrapeResult$new(
        {
          let rustString = url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          if let val = result {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(error) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }()))
  }
}
public class BatchScrapeResultRefMut: BatchScrapeResultRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class BatchScrapeResultRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension BatchScrapeResultRef {
  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$BatchScrapeResult$url(ptr))
  }

  public func result() -> ScrapeResult? {
    {
      let val = __swift_bridge__$BatchScrapeResult$result(ptr)
      if val != nil { return ScrapeResult(ptr: val!) } else { return nil }
    }()
  }

  public func error() -> RustString? {
    {
      let val = __swift_bridge__$BatchScrapeResult$error(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }
}
extension BatchScrapeResult: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_BatchScrapeResult$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_BatchScrapeResult$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BatchScrapeResult) {
    __swift_bridge__$Vec_BatchScrapeResult$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_BatchScrapeResult$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (BatchScrapeResult(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> BatchScrapeResultRef?
  {
    let pointer = __swift_bridge__$Vec_BatchScrapeResult$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BatchScrapeResultRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> BatchScrapeResultRefMut?
  {
    let pointer = __swift_bridge__$Vec_BatchScrapeResult$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BatchScrapeResultRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    BatchScrapeResultRef
  > {
    UnsafePointer<BatchScrapeResultRef>(
      OpaquePointer(__swift_bridge__$Vec_BatchScrapeResult$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_BatchScrapeResult$len(vecPtr)
  }
}

public class BatchCrawlResult: BatchCrawlResultRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$BatchCrawlResult$_free(ptr)
    }
  }
}
extension BatchCrawlResult {
  public convenience init<GenericIntoRustString: IntoRustString>(
    _ url: GenericIntoRustString, _ result: CrawlResult?, _ error: GenericIntoRustString?
  ) {
    self.init(
      ptr: __swift_bridge__$BatchCrawlResult$new(
        {
          let rustString = url.intoRustString()
          rustString.isOwned = false
          return rustString.ptr
        }(),
        {
          if let val = result {
            val.isOwned = false
            return val.ptr
          } else {
            return nil
          }
        }(),
        {
          if let rustString = optionalStringIntoRustString(error) {
            rustString.isOwned = false
            return rustString.ptr
          } else {
            return nil
          }
        }()))
  }
}
public class BatchCrawlResultRefMut: BatchCrawlResultRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class BatchCrawlResultRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension BatchCrawlResultRef {
  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$BatchCrawlResult$url(ptr))
  }

  public func result() -> CrawlResult? {
    {
      let val = __swift_bridge__$BatchCrawlResult$result(ptr)
      if val != nil { return CrawlResult(ptr: val!) } else { return nil }
    }()
  }

  public func error() -> RustString? {
    {
      let val = __swift_bridge__$BatchCrawlResult$error(ptr)
      if val != nil { return RustString(ptr: val!) } else { return nil }
    }()
  }
}
extension BatchCrawlResult: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_BatchCrawlResult$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_BatchCrawlResult$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BatchCrawlResult) {
    __swift_bridge__$Vec_BatchCrawlResult$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_BatchCrawlResult$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (BatchCrawlResult(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> BatchCrawlResultRef?
  {
    let pointer = __swift_bridge__$Vec_BatchCrawlResult$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BatchCrawlResultRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> BatchCrawlResultRefMut?
  {
    let pointer = __swift_bridge__$Vec_BatchCrawlResult$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BatchCrawlResultRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    BatchCrawlResultRef
  > {
    UnsafePointer<BatchCrawlResultRef>(
      OpaquePointer(__swift_bridge__$Vec_BatchCrawlResult$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_BatchCrawlResult$len(vecPtr)
  }
}

public class BrowserMode: BrowserModeRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$BrowserMode$_free(ptr)
    }
  }
}
public class BrowserModeRefMut: BrowserModeRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class BrowserModeRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension BrowserModeRef {
  public func to_string() -> RustString {
    RustString(ptr: __swift_bridge__$BrowserMode$to_string(ptr))
  }
}
extension BrowserMode: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_BrowserMode$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_BrowserMode$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BrowserMode) {
    __swift_bridge__$Vec_BrowserMode$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_BrowserMode$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (BrowserMode(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> BrowserModeRef? {
    let pointer = __swift_bridge__$Vec_BrowserMode$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BrowserModeRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> BrowserModeRefMut?
  {
    let pointer = __swift_bridge__$Vec_BrowserMode$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BrowserModeRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    BrowserModeRef
  > {
    UnsafePointer<BrowserModeRef>(OpaquePointer(__swift_bridge__$Vec_BrowserMode$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_BrowserMode$len(vecPtr)
  }
}

public class BrowserWait: BrowserWaitRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$BrowserWait$_free(ptr)
    }
  }
}
public class BrowserWaitRefMut: BrowserWaitRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class BrowserWaitRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension BrowserWaitRef {
  public func to_string() -> RustString {
    RustString(ptr: __swift_bridge__$BrowserWait$to_string(ptr))
  }
}
extension BrowserWait: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_BrowserWait$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_BrowserWait$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BrowserWait) {
    __swift_bridge__$Vec_BrowserWait$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_BrowserWait$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (BrowserWait(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> BrowserWaitRef? {
    let pointer = __swift_bridge__$Vec_BrowserWait$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BrowserWaitRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> BrowserWaitRefMut?
  {
    let pointer = __swift_bridge__$Vec_BrowserWait$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BrowserWaitRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    BrowserWaitRef
  > {
    UnsafePointer<BrowserWaitRef>(OpaquePointer(__swift_bridge__$Vec_BrowserWait$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_BrowserWait$len(vecPtr)
  }
}

public class BrowserBackend: BrowserBackendRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$BrowserBackend$_free(ptr)
    }
  }
}
public class BrowserBackendRefMut: BrowserBackendRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class BrowserBackendRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension BrowserBackendRef {
  public func to_string() -> RustString {
    RustString(ptr: __swift_bridge__$BrowserBackend$to_string(ptr))
  }
}
extension BrowserBackend: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_BrowserBackend$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_BrowserBackend$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BrowserBackend) {
    __swift_bridge__$Vec_BrowserBackend$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_BrowserBackend$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (BrowserBackend(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> BrowserBackendRef?
  {
    let pointer = __swift_bridge__$Vec_BrowserBackend$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BrowserBackendRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> BrowserBackendRefMut?
  {
    let pointer = __swift_bridge__$Vec_BrowserBackend$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BrowserBackendRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    BrowserBackendRef
  > {
    UnsafePointer<BrowserBackendRef>(
      OpaquePointer(__swift_bridge__$Vec_BrowserBackend$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_BrowserBackend$len(vecPtr)
  }
}

public class AuthConfig: AuthConfigRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$AuthConfig$_free(ptr)
    }
  }
}
public class AuthConfigRefMut: AuthConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class AuthConfigRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension AuthConfigRef {
  public func to_string() -> RustString {
    RustString(ptr: __swift_bridge__$AuthConfig$to_string(ptr))
  }
}
extension AuthConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_AuthConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_AuthConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: AuthConfig) {
    __swift_bridge__$Vec_AuthConfig$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_AuthConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (AuthConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> AuthConfigRef? {
    let pointer = __swift_bridge__$Vec_AuthConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return AuthConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> AuthConfigRefMut?
  {
    let pointer = __swift_bridge__$Vec_AuthConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return AuthConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<AuthConfigRef>
  {
    UnsafePointer<AuthConfigRef>(OpaquePointer(__swift_bridge__$Vec_AuthConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_AuthConfig$len(vecPtr)
  }
}

public class LinkType: LinkTypeRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$LinkType$_free(ptr)
    }
  }
}
public class LinkTypeRefMut: LinkTypeRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class LinkTypeRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension LinkTypeRef {
  public func to_string() -> RustString {
    RustString(ptr: __swift_bridge__$LinkType$to_string(ptr))
  }
}
extension LinkType: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_LinkType$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_LinkType$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: LinkType) {
    __swift_bridge__$Vec_LinkType$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_LinkType$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (LinkType(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> LinkTypeRef? {
    let pointer = __swift_bridge__$Vec_LinkType$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return LinkTypeRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> LinkTypeRefMut?
  {
    let pointer = __swift_bridge__$Vec_LinkType$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return LinkTypeRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<LinkTypeRef> {
    UnsafePointer<LinkTypeRef>(OpaquePointer(__swift_bridge__$Vec_LinkType$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_LinkType$len(vecPtr)
  }
}

public class ImageSource: ImageSourceRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ImageSource$_free(ptr)
    }
  }
}
public class ImageSourceRefMut: ImageSourceRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ImageSourceRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ImageSourceRef {
  public func to_string() -> RustString {
    RustString(ptr: __swift_bridge__$ImageSource$to_string(ptr))
  }
}
extension ImageSource: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ImageSource$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ImageSource$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ImageSource) {
    __swift_bridge__$Vec_ImageSource$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_ImageSource$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ImageSource(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> ImageSourceRef? {
    let pointer = __swift_bridge__$Vec_ImageSource$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ImageSourceRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> ImageSourceRefMut?
  {
    let pointer = __swift_bridge__$Vec_ImageSource$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ImageSourceRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    ImageSourceRef
  > {
    UnsafePointer<ImageSourceRef>(OpaquePointer(__swift_bridge__$Vec_ImageSource$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ImageSource$len(vecPtr)
  }
}

public class FeedType: FeedTypeRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$FeedType$_free(ptr)
    }
  }
}
public class FeedTypeRefMut: FeedTypeRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class FeedTypeRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension FeedTypeRef {
  public func to_string() -> RustString {
    RustString(ptr: __swift_bridge__$FeedType$to_string(ptr))
  }
}
extension FeedType: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_FeedType$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_FeedType$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: FeedType) {
    __swift_bridge__$Vec_FeedType$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_FeedType$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (FeedType(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> FeedTypeRef? {
    let pointer = __swift_bridge__$Vec_FeedType$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return FeedTypeRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> FeedTypeRefMut?
  {
    let pointer = __swift_bridge__$Vec_FeedType$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return FeedTypeRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<FeedTypeRef> {
    UnsafePointer<FeedTypeRef>(OpaquePointer(__swift_bridge__$Vec_FeedType$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_FeedType$len(vecPtr)
  }
}

public class AssetCategory: AssetCategoryRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$AssetCategory$_free(ptr)
    }
  }
}
public class AssetCategoryRefMut: AssetCategoryRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class AssetCategoryRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension AssetCategoryRef {
  public func to_string() -> RustString {
    RustString(ptr: __swift_bridge__$AssetCategory$to_string(ptr))
  }
}
extension AssetCategory: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_AssetCategory$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_AssetCategory$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: AssetCategory) {
    __swift_bridge__$Vec_AssetCategory$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_AssetCategory$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (AssetCategory(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> AssetCategoryRef?
  {
    let pointer = __swift_bridge__$Vec_AssetCategory$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return AssetCategoryRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> AssetCategoryRefMut?
  {
    let pointer = __swift_bridge__$Vec_AssetCategory$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return AssetCategoryRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    AssetCategoryRef
  > {
    UnsafePointer<AssetCategoryRef>(
      OpaquePointer(__swift_bridge__$Vec_AssetCategory$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_AssetCategory$len(vecPtr)
  }
}

public class PageAction: PageActionRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$PageAction$_free(ptr)
    }
  }
}
public class PageActionRefMut: PageActionRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class PageActionRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension PageActionRef {
  public func to_string() -> RustString {
    RustString(ptr: __swift_bridge__$PageAction$to_string(ptr))
  }
}
extension PageAction: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_PageAction$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_PageAction$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: PageAction) {
    __swift_bridge__$Vec_PageAction$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_PageAction$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (PageAction(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> PageActionRef? {
    let pointer = __swift_bridge__$Vec_PageAction$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return PageActionRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> PageActionRefMut?
  {
    let pointer = __swift_bridge__$Vec_PageAction$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return PageActionRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<PageActionRef>
  {
    UnsafePointer<PageActionRef>(OpaquePointer(__swift_bridge__$Vec_PageAction$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_PageAction$len(vecPtr)
  }
}

public class ScrollDirection: ScrollDirectionRefMut {
  var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ScrollDirection$_free(ptr)
    }
  }
}
public class ScrollDirectionRefMut: ScrollDirectionRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ScrollDirectionRef {
  var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ScrollDirectionRef {
  public func to_string() -> RustString {
    RustString(ptr: __swift_bridge__$ScrollDirection$to_string(ptr))
  }
}
extension ScrollDirection: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ScrollDirection$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ScrollDirection$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ScrollDirection) {
    __swift_bridge__$Vec_ScrollDirection$push(
      vecPtr,
      {
        value.isOwned = false
        return value.ptr
      }())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
    let pointer = __swift_bridge__$Vec_ScrollDirection$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ScrollDirection(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> ScrollDirectionRef?
  {
    let pointer = __swift_bridge__$Vec_ScrollDirection$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ScrollDirectionRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt)
    -> ScrollDirectionRefMut?
  {
    let pointer = __swift_bridge__$Vec_ScrollDirection$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ScrollDirectionRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
    ScrollDirectionRef
  > {
    UnsafePointer<ScrollDirectionRef>(
      OpaquePointer(__swift_bridge__$Vec_ScrollDirection$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ScrollDirection$len(vecPtr)
  }
}
