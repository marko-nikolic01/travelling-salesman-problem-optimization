from pydantic import BaseModel
from .cities import Cities
from .test_case import TestCase
from typing import List


class Result(BaseModel):
    cities: Cities
    sequential_test_cases: List[TestCase]
    parallel_test_cases: List[TestCase]