import typer
import uvicorn

from ._version import get_version
from .proxy import app

cli = typer.Typer(no_args_is_help=True)


@cli.command()
def run(
    port: str = typer.Option(8080, help="The port to run the proxy on."),
):
    """
    Start the OTEL-proxy.

    ARGS:
        Port: The port to run the proxy on.
    """
    typer.echo(f"Starting the proxy on port {port}")
    uvicorn.run(app, host="0.0.0.0", port=int(port), reload=False, log_level="trace")


def version_callback(value: bool):
    if value:
        typer.echo(f"{get_version()}")
        raise typer.Exit()


@cli.callback()
def main(version: bool = typer.Option(None, "--version", callback=version_callback)):
    """
    Otel Proxy Entry Point
    """


cli()
