from pydantic import BaseModel
from typing import List

class Cities(BaseModel):
    cities: List[str]
    distances: List[List[int]]