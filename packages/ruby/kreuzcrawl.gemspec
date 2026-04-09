# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = "kreuzcrawl"
  spec.version = "0.1.0"
  spec.authors = ["Na'aman Hirschfeld"]
  spec.email = ["naaman@kreuzberg.dev"]

  spec.summary = "High-performance web crawling engine"
  spec.description = <<~DESC
    Kreuzcrawl is a high-performance web crawling engine with a Rust core and native
    Ruby bindings via Magnus. Features async crawling, content scraping, and spider
    capabilities for efficient web data extraction.
  DESC
  spec.homepage = "https://github.com/kreuzberg-dev/kreuzcrawl"
  spec.license = "Elastic-2.0"
  spec.required_ruby_version = ">= 3.2.0"

  spec.metadata = {
    'homepage_uri' => spec.homepage,
    'source_code_uri' => 'https://github.com/kreuzberg-dev/kreuzcrawl',
    'changelog_uri' => 'https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/CHANGELOG.md',
    'bug_tracker_uri' => 'https://github.com/kreuzberg-dev/kreuzcrawl/issues',
    'rubygems_mfa_required' => 'true'
  }

  spec.files = Dir["lib/**/*.rb", "ext/**/*.{rs,toml,rb,lock}"]
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/kreuzcrawl_rb/extconf.rb"]

  spec.add_dependency "rb_sys", "~> 0.9"
end
