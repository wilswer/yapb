from typing import Optional

class ProgressBar:
    length: int
    current: int
    message: str
    description: str
    character: str
    head: str
    done: bool
    max_length: int

    def __init__(self, length: int, message: str, description: str) -> None:
        """Initialize the progress bar."""
        ...
    def get_length(self) -> int:
        """Get the length of the progress bar."""
        ...
    def update(self, message: Optional[str], description: Optional[str]) -> None:
        """Update the progress bar."""
        ...
    def render(self) -> None:
        """Render the progress bar."""
        ...
