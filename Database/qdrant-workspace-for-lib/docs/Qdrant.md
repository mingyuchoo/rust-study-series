# Qdrant

Qdrant는 벡터 데이터베이스로, 벡터 데이터를 효율적으로 저장하고 검색할 수 있도록 설계되었습니다.

## 개념

### Collection

Collection은 명명된 포인트 그룹입니다.

- Distance
- Point

### Distance Metric

Distance는 벡터 간의 유사성을 계산하는 방법을 나타냅니다.

- Dot
- Cosine
- Euclidean

### Point

Point는 벡터 데이터와 함께 저장되는 추가 정보를 나타냅니다.

- A vector (embedding)
- An optional unique ID
- An optional payload (metadata)

### Storage Types

Qdrant는 여러 가지 저장 타입을 지원합니다.

- In-Memory Storage
- Memmap Storage
