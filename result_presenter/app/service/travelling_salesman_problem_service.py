import subprocess
from app.dto import CitiesDTO, TestCaseDTO, PathItemDTO
from typing import List

class TravellingSalesmanProblemService:
    def __init__(self):
        self.input = "./resources/input/input.txt"
        self.sequential_bin = "./travelling_salesman_problem/travelling_salesman_problem_sequential"
        self.parallel_bin = "./travelling_salesman_problem/travelling_salesman_problem_parallel"
        self.sequential_output_template = "./resources/output/output_sequential_{}.txt"
        self.parallel_output_template = "./resources/output/output_parallel_{}.txt"
    
    def test(self, cities_dto: CitiesDTO, mode: str = "sequential") -> List[TestCaseDTO]:
        test_cases = []
        cities_only = [c.split(",")[0] for c in cities_dto.cities]

        if mode == "sequential":
            binary = self.sequential_bin
            output_template = self.sequential_output_template
        elif mode == "parallel":
            binary = self.parallel_bin
            output_template = self.parallel_output_template

        for n in range(2, len(cities_only) + 1):
            cities_subset = cities_only[:n]

            subprocess.run([binary] + cities_subset, check=True)
            output_file = output_template.format(n)

            with open(output_file, "r") as f:
                lines = f.read().splitlines()
                execution_time = float(lines[0].strip())
                shortest_distance = int(lines[1].strip())
                path_items = []
                for item_line in lines[2:]:
                    origin_city, origin_country, dest_city, dest_country, distance = item_line.strip().split(",")
                    path_items.append(PathItemDTO(
                        origin=f"{origin_city},{origin_country}",
                        destination=f"{dest_city},{dest_country}",
                        distance=int(distance)
                    ))

            test_cases.append(TestCaseDTO(
                cities=cities_subset,
                num_cities=n,
                shortest_distance=shortest_distance,
                execution_time=execution_time,
                shortest_path=path_items
            ))

        return test_cases
