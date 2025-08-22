from app.model import Result
import matplotlib.pyplot as plt

class ResultAnalysisService:
    def __init__(self):
        self.execution_times_path = "./results/plots/execution_times.png"
        self.speed_ups_path = "./results/plots/speed_ups.png"
        self.distance_ratio_path = "./results/plots/distance_ratio.png"
        self.time_per_edge_path = "./results/plots/time_per_edge.png"

    def analyze(self, result: Result):
        print("\nAnalyzing results...", flush=True)

        self.analyze_execution_times(result)
        self.analyze_speedup(result)

        print("\nAnalysis finished...", flush=True)

    def analyze_execution_times(self, result: Result):
        print("\nAnalyzing execution times...", flush=True)

        n_values = [tc.num_cities for tc in result.sequential_test_cases]
        seq_times = [tc.execution_time for tc in result.sequential_test_cases]
        par_times = [tc.execution_time for tc in result.parallel_test_cases]

        plt.figure(figsize=(8,6))
        plt.plot(n_values, seq_times, marker='o', color='red', label="Sequential")
        plt.plot(n_values, par_times, marker='o', color='green', label="Parallel")
        plt.xlabel("Number of cities (n)")
        plt.ylabel("Execution time (seconds)")
        plt.title("Execution time vs number of cities")
        plt.legend()
        plt.grid(True)
        plt.savefig(self.execution_times_path)
        plt.close()

        print("Execution times analysis finished", flush=True)

    def analyze_speedup(self, result: Result):
        print("\nAnalyzing speed-ups...", flush=True)

        n_values = [tc.num_cities for tc in result.sequential_test_cases]
        seq_times = [tc.execution_time for tc in result.sequential_test_cases]
        par_times = [tc.execution_time for tc in result.parallel_test_cases]

        speedup = [s / p for s, p in zip(seq_times, par_times)]

        plt.figure(figsize=(8,6))
        plt.plot(n_values, speedup, marker='o', color='blue')
        plt.xlabel("Number of cities (n)")
        plt.ylabel("Speedup (sequential / parallel)")
        plt.title("Speedup vs number of cities")
        plt.grid(True)
        plt.savefig(self.speed_ups_path)
        plt.close()

        print("Speed-ups analysis finished", flush=True)
