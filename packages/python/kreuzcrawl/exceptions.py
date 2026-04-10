"""Exception hierarchy."""

from __future__ import annotations


class CrawlError(Exception):
    """Errors that can occur during crawling, scraping, or mapping operations."""


class NotFoundError(CrawlError):
    """The requested page was not found (HTTP 404)."""


class UnauthorizedError(CrawlError):
    """The request was unauthorized (HTTP 401)."""


class ForbiddenError(CrawlError):
    """The request was forbidden (HTTP 403)."""


class WafBlockedError(CrawlError):
    """The request was blocked by a WAF or bot protection (HTTP 403 with WAF indicators)."""


class CrawlTimeoutError(CrawlError):
    """The request timed out."""


class RateLimitedError(CrawlError):
    """The request was rate-limited (HTTP 429)."""


class ServerError(CrawlError):
    """A server error occurred (HTTP 5xx)."""


class BadGatewayError(CrawlError):
    """A bad gateway error occurred (HTTP 502)."""


class GoneError(CrawlError):
    """The resource is permanently gone (HTTP 410)."""


class CrawlConnectionError(CrawlError):
    """A connection error occurred."""


class DnsError(CrawlError):
    """A DNS resolution error occurred."""


class SslError(CrawlError):
    """An SSL/TLS error occurred."""


class DataLossError(CrawlError):
    """Data was lost or truncated during transfer."""


class BrowserError(CrawlError):
    """The browser failed to launch, connect, or navigate."""


class BrowserTimeoutError(CrawlError):
    """The browser page load or rendering timed out."""


class InvalidConfigError(CrawlError):
    """The provided configuration is invalid."""


class OtherError(CrawlError):
    """An unclassified error occurred."""


