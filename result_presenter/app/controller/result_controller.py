from fastapi import APIRouter, status
from app.service import ResultService
from app.dto import ResultDTO

router = APIRouter(prefix="/api/results", tags=["Results"])

@router.post("/", response_model=ResultDTO, status_code=status.HTTP_200_OK)
def get_result():
    result_service = ResultService()
    return result_service.get_result()
