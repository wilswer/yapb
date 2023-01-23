from __future__ import annotations

class ProgressBar:
    """Progress bar class."""

    def __init__(
        self,
        capacity: int,
        message: str = "",
        description: str = "",
        ncols: int | None = 50,
    ) -> None:
        """Initialize the progress bar."""
        ...
    def update(
        self, message: str | None = None, description: str | None = None
    ) -> None:
        """Update the progress bar."""
        ...
    def render(self) -> None:
        """Render the progress bar."""
        ...
