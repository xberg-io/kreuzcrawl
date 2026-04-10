# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name          = 'kreuzcrawl'
  spec.version       = '0.1.0'
  spec.authors       = ['Kreuzberg Team']
  spec.summary       = 'High-performance web crawling engine'
  spec.description   = 'High-performance web crawling engine'
  spec.homepage      = 'https://github.com/kreuzberg-dev/kreuzcrawl'
  spec.license       = 'Elastic-2.0'
  spec.required_ruby_version = '>= 3.2.0'

  spec.metadata = {
    'homepage_uri' => spec.homepage,
    'source_code_uri' => 'https://github.com/kreuzberg-dev/kreuzcrawl',
    'rubygems_mfa_required' => 'true',
    'keywords' => 'crawl,scrape,web,spider'
  }

  spec.files         = Dir.glob('{lib/**/*,ext/**/*}')
  spec.require_paths = ['lib']
  spec.extensions    = ['ext/kreuzcrawl/extconf.rb']
end
