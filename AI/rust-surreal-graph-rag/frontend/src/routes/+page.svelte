<!-- 홈 페이지 -->
<div class="page">
  <div class="hero">
    <h1>RAG AI</h1>
    <p class="subtitle">벡터 검색과 그래프 탐색을 결합한 지능형 질의응답 서비스</p>
    <div class="features">
      <div class="feature-card">
        <div class="feature-icon">Q</div>
        <h3>통합 질의응답</h3>
        <p>벡터 + 그래프 검색을 결합한 하이브리드 RAG</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon">V</div>
        <h3>벡터 검색</h3>
        <p>임베딩 기반 의미론적 유사도 검색</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon">G</div>
        <h3>그래프 검색</h3>
        <p>지식 그래프 경로 탐색 및 관계 추론</p>
      </div>
    </div>
  </div>
</div>

<style>
  .hero {
    text-align: center;
    padding: 64px 0 48px;
  }

  h1 {
    font-size: 48px;
    font-weight: 700;
    letter-spacing: -0.05em;
    color: var(--color-black);
    margin-bottom: 12px;
  }

  .subtitle {
    font-size: 16px;
    color: var(--color-gray-500);
    margin-bottom: 48px;
  }

  .features {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
    text-align: left;
  }

  .feature-card {
    background: var(--color-white);
    border: 1px solid var(--color-gray-200);
    border-radius: var(--radius-md);
    padding: 24px;
    transition:
      border-color var(--transition),
      box-shadow var(--transition);
  }

  .feature-card:hover {
    border-color: var(--color-gray-300);
    box-shadow: var(--shadow-md);
  }

  .feature-icon {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-gray-900);
    color: var(--color-white);
    border-radius: var(--radius-sm);
    font-size: 14px;
    font-weight: 700;
    margin-bottom: 16px;
  }

  .feature-card h3 {
    font-size: 15px;
    font-weight: 600;
    letter-spacing: -0.02em;
    color: var(--color-gray-900);
    margin-bottom: 6px;
  }

  .feature-card p {
    font-size: 13px;
    color: var(--color-gray-500);
    line-height: 1.5;
  }

  @media (max-width: 640px) {
    .features {
      grid-template-columns: 1fr;
    }

    h1 {
      font-size: 36px;
    }
  }
</style>
