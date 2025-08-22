from app.model import Result, Cities, TestCase
from .travelling_salesman_problem_service import TravellingSalesmanProblemService
from .result_analysis_service import ResultAnalysisService

class ResultService:
    def __init__(self):
        self.input = "./results/input/input.txt"
        self.travelling_salesman_problem_service = TravellingSalesmanProblemService()
        self.result_analysis_service = ResultAnalysisService()

    def get_result(self) -> Result:
        cities: Cities = self.load_cities_from_file(self.input)
        result = Result(
            cities=cities,
            sequential_test_cases=[],
            parallel_test_cases=[]
        )

        sequential_test_cases: list[TestCase] = self.travelling_salesman_problem_service.test(cities, mode="sequential")
        parallel_test_cases: list[TestCase] = self.travelling_salesman_problem_service.test(cities, mode="parallel")

        result = Result(
            cities=cities,
            sequential_test_cases=sequential_test_cases,
            parallel_test_cases=parallel_test_cases
        )
    
        self.result_analysis_service.analyze(result)

        return result

    def load_cities_from_file(self, file_path: str) -> Cities:
        print("\nLoading input (cities and distances)...", flush=True)

        cities_set = set()
        edges = []

        with open(file_path, "r") as f:
            for line in f:
                parts = line.strip().split(",")
                if len(parts) != 5:
                    continue
                city1, country1, city2, country2, distance = parts
                full_city1 = f"{city1},{country1}"
                full_city2 = f"{city2},{country2}"
                cities_set.add(full_city1)
                cities_set.add(full_city2)
                edges.append((full_city1, full_city2, int(distance)))

        cities_list = sorted(list(cities_set))
        n = len(cities_list)
        city_index = {city: idx for idx, city in enumerate(cities_list)}

        distances = [[0 if i == j else float("inf") for j in range(n)] for i in range(n)]

        for city1, city2, distance in edges:
            i, j = city_index[city1], city_index[city2]
            distances[i][j] = distance
            distances[j][i] = distance


        print("Input (cities and distances) loaded...", flush=True)

        return Cities(cities=cities_list, distances=distances)
