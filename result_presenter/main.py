import subprocess
import matplotlib.pyplot as plt

cities_list = [
    "Lyon", "Paris", "Madrid", "Barcelona", "London", "Geneva", "Zurich",
    "Milan", "Florence", "Rome", "Venice", "Zagreb", "Budapest", "Vienna",
    "Prague", "Berlin", "Munich", "Frankfurt", "Brussels", "Amsterdam"
]

sequential_bin = "../travelling_salesman_problem_sequential/target/debug/travelling_salesman_problem_sequential"
parallel_bin = "../travelling_salesman_problem_parallel/target/debug/travelling_salesman_problem_parallel"

sequential_output_template = "../resources/output/output_sequential_{}.txt"
parallel_output_template = "../resources/output/output_parallel_{}.txt"

sequential_times = []
parallel_times = []
ns = list(range(2, len(cities_list) + 1))

for n in ns:
    cities = cities_list[:n]

    subprocess.run([sequential_bin] + cities, check=True)
    with open(sequential_output_template.format(n), "r") as f:
        seq_time = float(f.readline().strip())
    sequential_times.append(seq_time)

    subprocess.run([parallel_bin] + cities, check=True)
    with open(parallel_output_template.format(n), "r") as f:
        par_time = float(f.readline().strip())
    parallel_times.append(par_time)

plt.plot(ns, sequential_times, marker="o", label="Sequential")
plt.plot(ns, parallel_times, marker="o", label="Parallel")
plt.xlabel("Number of cities (n)")
plt.ylabel("Execution time (seconds)")
plt.title("Travelling Salesman Problem: Sequential vs Parallel")
plt.legend()
plt.grid(True)
plt.show()
