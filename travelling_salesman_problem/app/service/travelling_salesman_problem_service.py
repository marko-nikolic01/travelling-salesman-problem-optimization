import subprocess
from app.model import Cities, TestCase, PathItem
from typing import List

class TravellingSalesmanProblemService:
    def __init__(self):
        self.input = "./results/input/input.txt"
        self.sequential_bin = "./travelling_salesman_problem/travelling_salesman_problem_sequential"
        self.parallel_bin = "./travelling_salesman_problem/travelling_salesman_problem_parallel"
        self.sequential_output_template = "./results/output/output_sequential_{}.txt"
        self.parallel_output_template = "./results/output/output_parallel_{}.txt"
    
    def test(self, cities_dto: Cities, mode: str = "sequential") -> List[TestCase]:
        test_cases = []
        cities_only = [c.split(",")[0] for c in cities_dto.cities]

        if mode == "sequential":
            binary = self.sequential_bin
            cmd = [binary]
            output_template = self.sequential_output_template
        elif mode == "parallel":
            binary = self.parallel_bin
            cmd = [binary]
            output_template = self.parallel_output_template

        for n in range(2, len(cities_only) + 1):
            print(f"\nExecuting test case\tMODE: {mode.upper()}\tCITIES:{n}...", flush=True)

            cities_subset = cities_only[:n]

            subprocess.run(cmd + cities_subset, check=True)
            output_file = output_template.format(n)

            with open(output_file, "r") as f:
                lines = f.read().splitlines()
                execution_time = float(lines[0].strip())
                shortest_distance = int(lines[1].strip())
                path_items = []
                for item_line in lines[2:]:
                    origin_city, origin_country, dest_city, dest_country, distance = item_line.strip().split(",")
                    path_items.append(PathItem(
                        origin=f"{origin_city},{origin_country}",
                        destination=f"{dest_city},{dest_country}",
                        distance=int(distance)
                    ))

            test_cases.append(TestCase(
                cities=cities_subset,
                num_cities=n,
                shortest_distance=shortest_distance,
                execution_time=execution_time,
                shortest_path=path_items
            ))

            print(f"Test case executed", flush=True)

        return test_cases
