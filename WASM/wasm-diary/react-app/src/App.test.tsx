import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import App from "./App";

// DiaryManager 모킹 — 인메모리 저장소
let mockEntries: Record<string, unknown>[] = [];

const mockManager = {
  load_from_json: vi.fn((json: string) => {
    try {
      mockEntries = JSON.parse(json);
    } catch {
      /* 무시 */
    }
  }),
  save_to_json: vi.fn(() => JSON.stringify(mockEntries)),
  create_entry: vi.fn((title: string, content: string, mood: number) => {
    const entry = {
      id: `mock-${Date.now()}`,
      title,
      content,
      mood: ["Happy", "Sad", "Angry", "Anxious", "Calm", "Excited", "Tired", "Grateful"][mood],
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };
    mockEntries.push(entry);
    return JSON.stringify(entry);
  }),
  update_entry: vi.fn(
    (id: string, title: string, content: string, mood: number) => {
      const entry = mockEntries.find((e) => (e as { id: string }).id === id);
      if (!entry) return "";
      Object.assign(entry, {
        title,
        content,
        mood: ["Happy", "Sad", "Angry", "Anxious", "Calm", "Excited", "Tired", "Grateful"][mood],
        updated_at: new Date().toISOString(),
      });
      return JSON.stringify(entry);
    }
  ),
  delete_entry: vi.fn((id: string) => {
    const before = mockEntries.length;
    mockEntries = mockEntries.filter(
      (e) => (e as { id: string }).id !== id
    );
    return mockEntries.length < before;
  }),
  get_entry: vi.fn((id: string) => {
    const entry = mockEntries.find((e) => (e as { id: string }).id === id);
    return entry ? JSON.stringify(entry) : "";
  }),
  get_all_entries: vi.fn(() => JSON.stringify(mockEntries)),
  search_by_keyword: vi.fn((keyword: string) => {
    const kw = keyword.toLowerCase();
    return JSON.stringify(
      mockEntries.filter(
        (e) =>
          ((e as { title: string }).title ?? "").toLowerCase().includes(kw) ||
          ((e as { content: string }).content ?? "").toLowerCase().includes(kw)
      )
    );
  }),
  filter_by_mood: vi.fn((mood: number) => {
    const moodName = ["Happy", "Sad", "Angry", "Anxious", "Calm", "Excited", "Tired", "Grateful"][mood];
    return JSON.stringify(
      mockEntries.filter((e) => (e as { mood: string }).mood === moodName)
    );
  }),
  filter_by_date_range: vi.fn(() => JSON.stringify(mockEntries)),
  get_statistics: vi.fn(() =>
    JSON.stringify({
      total_entries: mockEntries.length,
      total_characters: 0,
      total_words: 0,
      mood_distribution: {},
    })
  ),
  free: vi.fn(),
};

vi.mock("wasm-lib", () => ({
  default: vi.fn(() => Promise.resolve()),
  DiaryManager: vi.fn(() => mockManager),
  Mood: { Happy: 0, Sad: 1, Angry: 2, Anxious: 3, Calm: 4, Excited: 5, Tired: 6, Grateful: 7 },
}));

// localStorage 모킹
const localStorageMock = (() => {
  let store: Record<string, string> = {};
  return {
    getItem: vi.fn((key: string) => store[key] ?? null),
    setItem: vi.fn((key: string, value: string) => {
      store[key] = value;
    }),
    clear: () => {
      store = {};
    },
  };
})();
Object.defineProperty(window, "localStorage", { value: localStorageMock });

// validate 정적 메서드 모킹
(mockManager as unknown as { validate: (t: string, c: string) => string }).validate = undefined as never;
vi.mocked(vi.fn()).mockImplementation;
const { DiaryManager: MockDiaryManager } = await import("wasm-lib");
(MockDiaryManager as unknown as Record<string, unknown>).validate = vi.fn(
  (title: string, content: string) => {
    const errors = [];
    if (!title.trim()) errors.push({ field: "title", message: "제목을 입력해주세요." });
    if (!content.trim()) errors.push({ field: "content", message: "내용을 입력해주세요." });
    return JSON.stringify({ valid: errors.length === 0, errors });
  }
);

describe("App", () => {
  beforeEach(() => {
    mockEntries = [];
    localStorageMock.clear();
    vi.clearAllMocks();
  });

  it("로딩 후 '나의 일기장' 헤더를 표시한다", async () => {
    render(<App />);
    expect(await screen.findByText("나의 일기장")).toBeInTheDocument();
  });

  it("일기가 없으면 빈 메시지를 표시한다", async () => {
    render(<App />);
    expect(await screen.findByText("작성된 일기가 없습니다.")).toBeInTheDocument();
  });

  it("새 일기 버튼을 누르면 작성 폼이 나타난다", async () => {
    const user = userEvent.setup();
    render(<App />);
    await screen.findByText("나의 일기장");
    await user.click(screen.getByText("새 일기"));
    expect(screen.getByText("새 일기 작성")).toBeInTheDocument();
  });

  it("통계 버튼을 누르면 통계 화면이 나타난다", async () => {
    const user = userEvent.setup();
    render(<App />);
    await screen.findByText("나의 일기장");
    await user.click(screen.getByText("통계"));
    expect(screen.getByText("총 일기 수")).toBeInTheDocument();
  });
});
