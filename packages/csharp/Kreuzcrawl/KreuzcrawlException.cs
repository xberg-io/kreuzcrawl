#nullable enable

using System;

namespace Kreuzcrawl;

public class KreuzcrawlException : Exception
{
    public int Code { get; }

    public KreuzcrawlException(int code, string message) : base(message)
    {
        Code = code;
    }

    public KreuzcrawlException(string message) : base(message)
    {
        Code = 0;
    }

    public KreuzcrawlException(string message, Exception innerException) : base(message, innerException)
    {
        Code = 0;
    }
}
