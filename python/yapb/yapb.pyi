from typing import Optional

class ProgressBar:
    """Progress bar class."""

    def __init__(
        self,
        capacity: int,
        message: str = "",
        description: str = "",
        ncols: Optional[int] = 50,
    ) -> None:
        """Initialize the progress bar."""
        ...
    def update(self, message: Optional[str], description: Optional[str]) -> None:
        """Update the progress bar."""
        ...
    def render(self) -> None:
        """Render the progress bar."""
        ...
