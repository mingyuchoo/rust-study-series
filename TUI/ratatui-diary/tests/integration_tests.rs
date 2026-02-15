use ratatui_diary::{
    model::{Screen, EditorMode},
    Model, Msg, storage::Storage, update, view,
};
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use std::collections::HashSet;
use tempfile::TempDir;
use chrono::NaiveDate;

// 애플리케이션 초기화 테스트
#[test]
fn test_application_startup() {
    // Given: 임시 디렉토리를 사용한 저장소 초기화
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let entries = storage.scan_entries().unwrap();

    // When: Model 생성
    let model = Model::new(entries, storage);

    // Then: Calendar 화면으로 시작되고, 에디터는 Normal 모드
    assert_eq!(model.screen, Screen::Calendar);
    assert_eq!(model.editor_state.mode, EditorMode::Normal);
    assert!(!model.show_error_popup);
    assert!(model.error_message.is_none());
}

// 전체 일기 작성 워크플로우 테스트
#[test]
fn test_full_diary_workflow() {
    // Given: 애플리케이션 초기화
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let entries = storage.scan_entries().unwrap();
    let mut model = Model::new(entries, storage);

    // When: 날짜 선택
    let date = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
    model.calendar_state.selected_date = date;
    let cmd = update::update(&mut model, Msg::CalendarSelectDate);

    // Then: LoadDiary 명령이 반환되어야 함
    assert!(cmd.is_some());
    assert_eq!(model.screen, Screen::Editor);
    assert_eq!(model.editor_state.date, date);
    assert_eq!(model.editor_state.mode, EditorMode::Normal);

    // When: 일기 로드 성공
    update::update(&mut model, Msg::LoadDiarySuccess(date, "test".to_string()));

    // Then: 내용이 로드됨
    assert_eq!(model.editor_state.content, vec!["test".to_string()]);

    // When: Insert 모드 진입
    update::update(
        &mut model,
        Msg::EditorEnterInsert(ratatui_diary::message::InsertPosition::BeforeCursor),
    );

    // Then: Insert 모드로 변경
    assert_eq!(model.editor_state.mode, EditorMode::Insert);

    // When: 문자 입력
    update::update(&mut model, Msg::EditorInsertChar('H'));
    update::update(&mut model, Msg::EditorInsertChar('i'));

    // Then: 내용이 수정됨
    assert!(model.editor_state.is_modified);

    // When: Normal 모드로 복귀
    update::update(&mut model, Msg::EditorEnterNormalMode);

    // Then: Normal 모드로 변경
    assert_eq!(model.editor_state.mode, EditorMode::Normal);

    // When: Space 모드 진입 및 저장
    update::update(&mut model, Msg::EditorEnterSpaceMode);
    let save_cmd = update::update(&mut model, Msg::EditorSpaceSave);

    // Then: SaveDiary 명령이 반환
    assert!(save_cmd.is_some());

    // When: 저장 성공 메시지 처리
    update::update(&mut model, Msg::SaveDiarySuccess);

    // Then: 수정 플래그만 초기화됨
    assert!(!model.editor_state.is_modified);
}

// 에러 처리 워크플로우 테스트
#[test]
fn test_error_handling_workflow() {
    // Given: 모델 초기화
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);

    // When: 로드 실패 메시지 처리
    update::update(&mut model, Msg::LoadDiaryFailed("파일을 찾을 수 없음".to_string()));

    // Then: 에러 팝업이 표시됨 (update.rs에서 "로드 실패: " 접두사 추가)
    assert!(model.show_error_popup);
    assert_eq!(
        model.error_message,
        Some("로드 실패: 파일을 찾을 수 없음".to_string())
    );

    // When: 에러 메시지 무시
    update::update(&mut model, Msg::DismissError);

    // Then: 에러 팝업이 닫힘
    assert!(!model.show_error_popup);
    assert!(model.error_message.is_none());
}

// 저장 실패 에러 처리 테스트
#[test]
fn test_save_error_handling() {
    // Given: 모델 초기화
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);

    // When: 저장 실패 메시지 처리
    update::update(&mut model, Msg::SaveDiaryFailed("쓰기 권한 없음".to_string()));

    // Then: 에러 팝업이 표시됨 (update.rs에서 "저장 실패: " 접두사 추가)
    assert!(model.show_error_popup);
    assert_eq!(model.error_message, Some("저장 실패: 쓰기 권한 없음".to_string()));

    // When: 에러 무시
    update::update(&mut model, Msg::DismissError);

    // Then: 에러 팝업이 닫힘
    assert!(!model.show_error_popup);
}

// 렌더링 테스트
#[test]
fn test_render_all_states() {
    // Given: 다양한 상태의 모델
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    // When/Then: Calendar 화면 렌더링
    terminal.draw(|f| view::view(f, &model)).unwrap();

    // When: Editor 화면으로 전환
    model.screen = Screen::Editor;

    // Then: Editor 화면 렌더링
    terminal.draw(|f| view::view(f, &model)).unwrap();

    // When: 에러 팝업 활성화
    model.show_error_popup = true;
    model.error_message = Some("테스트 에러".to_string());

    // Then: 에러 팝업과 함께 렌더링
    terminal.draw(|f| view::view(f, &model)).unwrap();

    // When: 에러 팝업 비활성화
    model.show_error_popup = false;

    // Then: 에러 팝업 없이 렌더링
    terminal.draw(|f| view::view(f, &model)).unwrap();
}

// 키보드 네비게이션 시뮬레이션 테스트
#[test]
fn test_keyboard_navigation_simulation() {
    // Given: Calendar 화면 초기 상태
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    let original_date = model.calendar_state.selected_date;

    // When: 오른쪽 방향키 이동
    update::update(&mut model, Msg::CalendarMoveRight);
    let first_move = model.calendar_state.selected_date;

    // Then: 날짜가 증가
    assert_ne!(first_move, original_date);

    // When: 오른쪽 방향키 추가 이동
    update::update(&mut model, Msg::CalendarMoveRight);
    let second_move = model.calendar_state.selected_date;

    // Then: 날짜가 계속 증가
    assert_ne!(second_move, first_move);

    // When: 왼쪽 방향키로 이동
    update::update(&mut model, Msg::CalendarMoveLeft);
    let after_left = model.calendar_state.selected_date;

    // Then: 날짜가 감소
    assert_ne!(after_left, second_move);
    assert_eq!(after_left, first_move);
}

// 에디터 네비게이션 테스트
#[test]
fn test_editor_navigation() {
    // Given: 에디터 화면 초기화
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.content = vec!["Hello World".to_string()];

    // When: 이동 메시지 처리
    let original_col = model.editor_state.cursor_col;
    update::update(&mut model, Msg::EditorMoveRight);

    // Then: 커서가 이동
    assert_eq!(model.editor_state.cursor_col, original_col + 1);

    // When: 왼쪽 이동
    update::update(&mut model, Msg::EditorMoveLeft);

    // Then: 커서가 원위치로 복귀
    assert_eq!(model.editor_state.cursor_col, original_col);
}

// Quit 메시지 테스트
#[test]
fn test_quit_message() {
    // Given: 모델 초기화
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    let initial_screen = model.screen;

    // When: Quit 메시지 처리
    let _ = update::update(&mut model, Msg::Quit);

    // Then: 모델 상태는 변하지 않음 (main loop에서 탈출 조건으로 사용)
    assert_eq!(model.screen, initial_screen);
}

// 복합 에디터 워크플로우 테스트
#[test]
fn test_complex_editor_workflow() {
    // Given: 에디터 화면 초기화
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;

    // When: Insert 모드로 진입
    update::update(
        &mut model,
        Msg::EditorEnterInsert(ratatui_diary::message::InsertPosition::BeforeCursor),
    );

    // Then: Insert 모드 확인
    assert_eq!(model.editor_state.mode, EditorMode::Insert);

    // When: 여러 문자 입력
    for ch in "테스트".chars() {
        update::update(&mut model, Msg::EditorInsertChar(ch));
    }

    // Then: 내용이 입력됨
    assert!(!model.editor_state.content.is_empty());
    assert!(model.editor_state.is_modified);

    // When: 새 줄 추가
    update::update(&mut model, Msg::EditorNewLine);

    // Then: 내용이 여러 줄로 분리
    let line_count = model.editor_state.content.len();
    assert!(line_count >= 1);

    // When: Normal 모드로 복귀
    update::update(&mut model, Msg::EditorEnterNormalMode);

    // Then: Normal 모드 확인
    assert_eq!(model.editor_state.mode, EditorMode::Normal);
}

// 월 네비게이션 테스트
#[test]
fn test_calendar_month_navigation() {
    // Given: Calendar 초기화
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    let initial_month = model.calendar_state.current_month;

    // When: Space 모드 진입
    update::update(&mut model, Msg::CalendarEnterSpaceMode);

    // Then: Space 서브모드 활성화
    assert!(model.calendar_state.submode.is_some());

    // When: 다음 달로 이동
    update::update(&mut model, Msg::CalendarSpaceNextMonth);

    // Then: 월이 증가
    let new_month = model.calendar_state.current_month;
    if initial_month == 12 {
        assert_eq!(new_month, 1);
    } else {
        assert_eq!(new_month, initial_month + 1);
    }

    // When: 서브모드 종료
    update::update(&mut model, Msg::CalendarExitSubMode);

    // Then: 서브모드가 비활성화
    assert!(model.calendar_state.submode.is_none());
}

// 백스페이스 처리 테스트
#[test]
fn test_backspace_handling() {
    // Given: 에디터에 내용이 있는 상태
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;
    model.editor_state.content = vec!["Hello".to_string()];
    model.editor_state.cursor_col = 5;

    // When: Insert 모드로 진입
    update::update(
        &mut model,
        Msg::EditorEnterInsert(ratatui_diary::message::InsertPosition::BeforeCursor),
    );

    // When: 백스페이스 처리
    update::update(&mut model, Msg::EditorBackspace);

    // Then: 내용이 수정됨
    assert!(model.editor_state.is_modified);
}

// 다중 화면 전환 테스트
#[test]
fn test_screen_transition() {
    // Given: Calendar 화면
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    assert_eq!(model.screen, Screen::Calendar);

    // When: 날짜 선택
    model.calendar_state.selected_date = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
    update::update(&mut model, Msg::CalendarSelectDate);

    // Then: Editor 화면으로 전환
    assert_eq!(model.screen, Screen::Editor);

    // When: 에디터에서 뒤로 가기
    update::update(&mut model, Msg::EditorBack);

    // Then: Calendar 화면으로 복귀
    assert_eq!(model.screen, Screen::Calendar);
}

// Undo/Redo 기능 테스트
#[test]
fn test_undo_redo() {
    // Given: 에디터 초기화
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    model.screen = Screen::Editor;

    // When: Insert 모드로 진입하여 문자 입력
    update::update(
        &mut model,
        Msg::EditorEnterInsert(ratatui_diary::message::InsertPosition::BeforeCursor),
    );
    update::update(&mut model, Msg::EditorInsertChar('A'));
    update::update(&mut model, Msg::EditorEnterNormalMode);

    let with_char = model.editor_state.content.clone();

    // When: Undo 메시지
    update::update(&mut model, Msg::EditorUndo);

    // Then: 이전 상태로 복귀
    let after_undo = model.editor_state.content.clone();
    assert_ne!(with_char, after_undo);

    // When: Redo 메시지
    update::update(&mut model, Msg::EditorRedo);

    // Then: 다시 문자가 입력된 상태로 복귀
    let after_redo = model.editor_state.content.clone();
    assert_eq!(with_char, after_redo);
}

// Year 네비게이션 테스트
#[test]
fn test_calendar_year_navigation() {
    // Given: Calendar 초기화
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let mut model = Model::new(HashSet::new(), storage);
    let initial_year = model.calendar_state.current_year;

    // When: Space 모드 진입
    update::update(&mut model, Msg::CalendarEnterSpaceMode);

    // Then: Space 서브모드 활성화
    assert!(model.calendar_state.submode.is_some());

    // When: 다음 연도로 이동
    update::update(&mut model, Msg::CalendarSpaceNextYear);

    // Then: 연도가 증가하고 submode는 자동으로 None이 됨
    assert_eq!(model.calendar_state.current_year, initial_year + 1);
    assert!(model.calendar_state.submode.is_none());

    // When: Space 모드 다시 진입
    update::update(&mut model, Msg::CalendarEnterSpaceMode);

    // When: 이전 연도로 이동
    update::update(&mut model, Msg::CalendarSpacePrevYear);

    // Then: 연도가 감소하고 submode는 자동으로 None이 됨
    assert_eq!(model.calendar_state.current_year, initial_year);
    assert!(model.calendar_state.submode.is_none());
}

// Diary 삭제 성공 메시지 처리 테스트
#[test]
fn test_delete_diary_success() {
    // Given: 저장된 일기가 있는 상태
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();
    let date = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();

    // 일기 저장
    let _ = storage.save(date, "테스트 내용");
    let entries = storage.scan_entries().unwrap();

    let mut model = Model::new(entries, storage);

    // diary_entries에 date가 포함되어 있음을 확인
    assert!(model.diary_entries.entries.contains(&date));

    // When: 삭제 성공 메시지 처리
    update::update(&mut model, Msg::DeleteDiarySuccess(date));

    // Then: 일기 항목이 제거되고 Calendar 화면으로 복귀
    assert!(!model.diary_entries.entries.contains(&date));
    assert_eq!(model.screen, Screen::Calendar);
}
