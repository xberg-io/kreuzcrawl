# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = 'kreuzcrawl'
  spec.version = '0.1.0-rc.8'
  spec.authors       = ['Kreuzberg Team']
  spec.summary       = 'High-performance web crawling engine'
  spec.description   = 'High-performance web crawling engine'
  spec.homepage      = 'https://github.com/kreuzberg-dev/kreuzcrawl'
  spec.license       = 'Elastic-2.0'
  spec.required_ruby_version = '>= 3.2.0'
  spec.metadata['keywords'] = %w[crawl scrape web spider].join(',')
  spec.metadata['rubygems_mfa_required'] = 'true'

  spec.files         = Dir.glob(['lib/**/*', 'ext/**/*'])
  spec.require_paths = ['lib']
  spec.extensions    = ['ext/kreuzcrawl_rb/extconf.rb']

  spec.add_dependency 'rb_sys', '~> 0.9'
end
