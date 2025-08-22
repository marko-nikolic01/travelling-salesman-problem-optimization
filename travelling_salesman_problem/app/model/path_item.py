from pydantic import BaseModel

class PathItem(BaseModel):
    origin: str
    destination: str
    distance: int