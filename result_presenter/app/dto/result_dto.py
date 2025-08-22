from pydantic import BaseModel
from .cities_dto import CitiesDTO
from .test_case_dto import TestCaseDTO
from typing import List


class ResultDTO(BaseModel):
    cities: CitiesDTO
    sequential_test_cases: List[TestCaseDTO]
    parallel_test_cases: List[TestCaseDTO]