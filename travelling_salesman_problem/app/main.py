from app.service import ResultService

print("Travelling salesman result presenter started...", flush=True)

result_service = ResultService()

result_service.get_result()

print("\nTravelling salesman result presenter finished", flush=True)
