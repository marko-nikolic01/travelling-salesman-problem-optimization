from pydantic import BaseModel
from .path_item import PathItem
from typing import List

class TestCase(BaseModel):
    cities: List[str]
    num_cities: int
    shortest_distance: int
    execution_time: float
    shortest_path: List[PathItem]