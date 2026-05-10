mix_config = Mix.Project.config
IO.inspect(mix_config)

engine_config = "{\"max_concurrent\":0}"
result = Kreuzcrawl.create_engine(engine_config)
IO.puts("Result: #{inspect(result)}")
