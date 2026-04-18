defmodule E2eElixir.MixProject do
  use Mix.Project

  def project do
    [
      app: :e2e_elixir,
      version: "0.1.0",
      elixir: "~> 1.14",
      deps: deps()
    ]
  end

  defp deps do
    [
      {:kreuzcrawl, "~> 0.1.0-rc.2"}
    ]
  end
end
