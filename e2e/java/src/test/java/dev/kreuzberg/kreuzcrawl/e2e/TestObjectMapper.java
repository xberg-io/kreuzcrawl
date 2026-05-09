package dev.kreuzberg.kreuzcrawl.e2e;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.PropertyNamingStrategies;
import com.fasterxml.jackson.datatype.jdk8.Jdk8Module;

/**
 * Provides a pre-configured ObjectMapper with snake_case to camelCase conversion for e2e tests.
 */
public final class TestObjectMapper {

    private TestObjectMapper() {
    }

    public static ObjectMapper createMapper() {
        return new ObjectMapper()
                .registerModule(new Jdk8Module())
                .setPropertyNamingStrategy(PropertyNamingStrategies.SNAKE_CASE);
    }
}
