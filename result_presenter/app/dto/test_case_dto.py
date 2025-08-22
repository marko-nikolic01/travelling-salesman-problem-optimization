from pydantic import BaseModel
from .path_item_dto import PathItemDTO
from typing import List

class TestCaseDTO(BaseModel):
    cities: List[str]
    num_cities: int
    shortest_distance: int
    execution_time: float
    shortest_path: List[PathItemDTO]