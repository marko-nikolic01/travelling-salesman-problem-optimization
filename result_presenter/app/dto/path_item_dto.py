from pydantic import BaseModel

class PathItemDTO(BaseModel):
    origin: str
    destination: str
    distance: int