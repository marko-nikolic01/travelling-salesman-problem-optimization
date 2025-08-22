from pydantic import BaseModel
from typing import List

class CitiesDTO(BaseModel):
    cities: List[str]
    distances: List[List[int]]