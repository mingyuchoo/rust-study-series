#!/usr/bin/env python3
"""
TDD Workflow Registry Manager
docs/registry.json 을 관리하는 유틸리티.
trace_map 을 통한 FR->SPEC->TC->IMPL 추적성 색인을 포함한다.
"""

import json
import sys
import os
from datetime import date


def load_registry(path="docs/registry.json"):
    """레지스트리 파일을 읽는다. 없으면 초기 구조를 반환한다."""
    if os.path.exists(path):
        with open(path, "r", encoding="utf-8") as f:
            data = json.load(f)
        # trace_map 이 없는 구버전 호환
        if "trace_map" not in data:
            data["trace_map"] = {}
        return data
    return {"last_prd_id": 0, "last_spec_id": 0, "entries": [], "trace_map": {}}


def save_registry(data, path="docs/registry.json"):
    """레지스트리 파일을 저장한다."""
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)


def next_prd_id(registry):
    """다음 PRD ID를 생성한다."""
    registry["last_prd_id"] += 1
    return f"PRD-{registry['last_prd_id']:03d}"


def next_spec_id(registry):
    """다음 SPEC ID를 생성한다."""
    registry["last_spec_id"] += 1
    return f"SPEC-{registry['last_spec_id']:03d}"


def add_prd_entry(registry, prd_id, title, fr_list):
    """PRD 항목을 추가하고 trace_map 을 초기화한다.

    Args:
        registry: 레지스트리 데이터
        prd_id: PRD ID (예: "PRD-001")
        title: PRD 제목
        fr_list: FR 목록. [{"id": "FR-1", "title": "기능 설명"}, ...]
    """
    entry = {
        "prd_id": prd_id,
        "title": title,
        "created_at": date.today().isoformat(),
        "status": "draft",
        "spec_ids": []
    }
    registry["entries"].append(entry)

    # trace_map 초기화
    registry["trace_map"][prd_id] = {"fr": {}}
    for fr in fr_list:
        registry["trace_map"][prd_id]["fr"][fr["id"]] = {
            "title": fr["title"],
            "specs": [],
            "test_cases": [],
            "test_files": [],
            "impl_files": [],
            "impl_symbols": [],
            "status": "draft"
        }

    return entry


def add_spec_entry(registry, spec_id, prd_id, title, fr_tc_mapping):
    """SPEC 항목을 추가하고 trace_map 의 FR->SPEC, FR->TC 매핑을 갱신한다.

    Args:
        registry: 레지스트리 데이터
        spec_id: SPEC ID (예: "SPEC-001")
        prd_id: 부모 PRD ID
        title: SPEC 제목
        fr_tc_mapping: FR->TC 매핑.
            {"FR-1": ["TC-1", "TC-2"], "FR-2": ["TC-3"]}
    """
    entry = {
        "spec_id": spec_id,
        "prd_id": prd_id,
        "title": title,
        "created_at": date.today().isoformat(),
        "status": "draft",
        "test_status": "pending"
    }
    registry["entries"].append(entry)

    # PRD 항목에 SPEC ID 연결
    for e in registry["entries"]:
        if e.get("prd_id") == prd_id and "spec_ids" in e:
            if spec_id not in e["spec_ids"]:
                e["spec_ids"].append(spec_id)
            break

    # trace_map 갱신: FR->SPEC, FR->TC
    if prd_id in registry["trace_map"]:
        prd_trace = registry["trace_map"][prd_id]["fr"]
        for fr_id, tc_list in fr_tc_mapping.items():
            if fr_id in prd_trace:
                if spec_id not in prd_trace[fr_id]["specs"]:
                    prd_trace[fr_id]["specs"].append(spec_id)
                for tc_id in tc_list:
                    tc_key = f"{spec_id}/{tc_id}"
                    if tc_key not in prd_trace[fr_id]["test_cases"]:
                        prd_trace[fr_id]["test_cases"].append(tc_key)

    return entry


def update_trace_test_files(registry, prd_id, fr_id, test_file):
    """trace_map 에 테스트 파일 경로를 추가한다."""
    if prd_id in registry["trace_map"]:
        fr_entry = registry["trace_map"][prd_id]["fr"].get(fr_id)
        if fr_entry and test_file not in fr_entry["test_files"]:
            fr_entry["test_files"].append(test_file)


def update_trace_impl(registry, prd_id, fr_id, impl_file, symbols):
    """trace_map 에 구현 파일과 심볼을 추가한다."""
    if prd_id in registry["trace_map"]:
        fr_entry = registry["trace_map"][prd_id]["fr"].get(fr_id)
        if fr_entry:
            if impl_file not in fr_entry["impl_files"]:
                fr_entry["impl_files"].append(impl_file)
            for sym in symbols:
                if sym not in fr_entry["impl_symbols"]:
                    fr_entry["impl_symbols"].append(sym)


def update_status(registry, item_id, status, test_status=None):
    """항목의 상태를 업데이트한다."""
    for entry in registry["entries"]:
        if entry.get("prd_id") == item_id or entry.get("spec_id") == item_id:
            entry["status"] = status
            if test_status and "test_status" in entry:
                entry["test_status"] = test_status
            return True
    return False


def update_fr_status(registry, prd_id, fr_id, status):
    """trace_map 에서 특정 FR의 상태를 업데이트한다."""
    if prd_id in registry["trace_map"]:
        fr_entry = registry["trace_map"][prd_id]["fr"].get(fr_id)
        if fr_entry:
            fr_entry["status"] = status
            return True
    return False


def check_prd_completion(registry, prd_id):
    """PRD의 모든 FR이 passed 상태인지 확인한다."""
    if prd_id not in registry["trace_map"]:
        return False
    frs = registry["trace_map"][prd_id]["fr"]
    if not frs:
        return False
    return all(fr["status"] == "passed" for fr in frs.values())


def get_trace_summary(registry, prd_id):
    """특정 PRD의 추적 현황 요약을 반환한다."""
    if prd_id not in registry["trace_map"]:
        return None

    frs = registry["trace_map"][prd_id]["fr"]
    total = len(frs)
    covered = sum(
        1 for fr in frs.values()
        if fr["specs"] and fr["test_cases"] and fr["impl_files"]
    )
    passed = sum(1 for fr in frs.values() if fr["status"] == "passed")

    return {
        "prd_id": prd_id,
        "total_frs": total,
        "covered_frs": covered,
        "passed_frs": passed,
        "total_tcs": sum(len(fr["test_cases"]) for fr in frs.values()),
        "total_impl_symbols": sum(len(fr["impl_symbols"]) for fr in frs.values()),
        "completeness": "COMPLETE" if covered == total else "INCOMPLETE",
    }


def recover_from_filesystem():
    """파일시스템에서 ID를 복원한다."""
    registry = {"last_prd_id": 0, "last_spec_id": 0, "entries": [], "trace_map": {}}

    if os.path.exists("docs/prd"):
        for f in sorted(os.listdir("docs/prd")):
            if f.startswith("PRD-") and f.endswith(".md"):
                try:
                    num = int(f[4:7])
                    registry["last_prd_id"] = max(registry["last_prd_id"], num)
                    registry["entries"].append({
                        "prd_id": f"PRD-{num:03d}",
                        "title": "(recovered)",
                        "created_at": date.today().isoformat(),
                        "status": "unknown",
                        "spec_ids": []
                    })
                except ValueError:
                    pass

    if os.path.exists("docs/spec"):
        for f in sorted(os.listdir("docs/spec")):
            if f.startswith("SPEC-") and f.endswith(".md"):
                try:
                    num = int(f[5:8])
                    registry["last_spec_id"] = max(registry["last_spec_id"], num)
                    registry["entries"].append({
                        "spec_id": f"SPEC-{num:03d}",
                        "prd_id": "(unknown)",
                        "title": "(recovered)",
                        "created_at": date.today().isoformat(),
                        "status": "unknown",
                        "test_status": "unknown"
                    })
                except ValueError:
                    pass

    return registry


def main():
    if len(sys.argv) < 2:
        print("Usage: registry.py <command> [args]")
        print("Commands: next-prd, next-spec, add-prd, add-spec,")
        print("          trace-test, trace-impl, fr-status,")
        print("          prd-complete, summary, status, recover, show")
        sys.exit(1)

    cmd = sys.argv[1]
    registry = load_registry()

    if cmd == "next-prd":
        prd_id = next_prd_id(registry)
        save_registry(registry)
        print(prd_id)

    elif cmd == "next-spec":
        spec_id = next_spec_id(registry)
        save_registry(registry)
        print(spec_id)

    elif cmd == "add-prd":
        # add-prd <title> <fr-json>
        # fr-json: [{"id": "FR-1", "title": "설명"}, ...]
        if len(sys.argv) < 4:
            print('Usage: registry.py add-prd <title> \'[{"id":"FR-1","title":"desc"}]\'')
            sys.exit(1)
        prd_id = next_prd_id(registry)
        fr_list = json.loads(sys.argv[3])
        add_prd_entry(registry, prd_id, sys.argv[2], fr_list)
        save_registry(registry)
        print(prd_id)

    elif cmd == "add-spec":
        # add-spec <PRD-ID> <title> <fr-tc-json>
        # fr-tc-json: {"FR-1": ["TC-1", "TC-2"], "FR-2": ["TC-3"]}
        if len(sys.argv) < 5:
            print('Usage: registry.py add-spec <PRD-ID> <title> \'{"FR-1":["TC-1"]}\'')
            sys.exit(1)
        spec_id = next_spec_id(registry)
        fr_tc = json.loads(sys.argv[4])
        add_spec_entry(registry, spec_id, sys.argv[2], sys.argv[3], fr_tc)
        save_registry(registry)
        print(spec_id)

    elif cmd == "trace-test":
        # trace-test <PRD-ID> <FR-ID> <test-file>
        if len(sys.argv) < 5:
            print("Usage: registry.py trace-test <PRD-ID> <FR-ID> <test-file>")
            sys.exit(1)
        update_trace_test_files(registry, sys.argv[2], sys.argv[3], sys.argv[4])
        save_registry(registry)
        print(f"Traced: {sys.argv[4]} -> {sys.argv[2]}/{sys.argv[3]}")

    elif cmd == "trace-impl":
        # trace-impl <PRD-ID> <FR-ID> <impl-file> <symbol1,symbol2>
        if len(sys.argv) < 6:
            print("Usage: registry.py trace-impl <PRD-ID> <FR-ID> <impl-file> <sym1,sym2>")
            sys.exit(1)
        symbols = [s.strip() for s in sys.argv[5].split(",")]
        update_trace_impl(registry, sys.argv[2], sys.argv[3], sys.argv[4], symbols)
        save_registry(registry)
        print(f"Traced: {sys.argv[4]} ({', '.join(symbols)}) -> {sys.argv[2]}/{sys.argv[3]}")

    elif cmd == "fr-status":
        # fr-status <PRD-ID> <FR-ID> <status>
        if len(sys.argv) < 5:
            print("Usage: registry.py fr-status <PRD-ID> <FR-ID> <status>")
            sys.exit(1)
        update_fr_status(registry, sys.argv[2], sys.argv[3], sys.argv[4])
        save_registry(registry)
        print(f"{sys.argv[2]}/{sys.argv[3]} -> {sys.argv[4]}")

    elif cmd == "prd-complete":
        # prd-complete <PRD-ID>
        if len(sys.argv) < 3:
            print("Usage: registry.py prd-complete <PRD-ID>")
            sys.exit(1)
        complete = check_prd_completion(registry, sys.argv[2])
        print(f"{sys.argv[2]}: {'COMPLETE' if complete else 'INCOMPLETE'}")

    elif cmd == "summary":
        # summary <PRD-ID>
        if len(sys.argv) < 3:
            print("Usage: registry.py summary <PRD-ID>")
            sys.exit(1)
        s = get_trace_summary(registry, sys.argv[2])
        if s:
            print(json.dumps(s, indent=2, ensure_ascii=False))
        else:
            print(f"PRD not found: {sys.argv[2]}")

    elif cmd == "status":
        if len(sys.argv) < 4:
            print("Usage: registry.py status <ID> <new-status> [test-status]")
            sys.exit(1)
        test_st = sys.argv[4] if len(sys.argv) > 4 else None
        update_status(registry, sys.argv[2], sys.argv[3], test_st)
        save_registry(registry)
        print(f"Updated {sys.argv[2]} -> {sys.argv[3]}")

    elif cmd == "recover":
        registry = recover_from_filesystem()
        save_registry(registry)
        print(f"Recovered: {registry['last_prd_id']} PRDs, {registry['last_spec_id']} SPECs")
        print("NOTE: trace_map 복구에는 verify_trace.py --fix 를 사용하세요.")

    elif cmd == "show":
        print(json.dumps(registry, indent=2, ensure_ascii=False))

    else:
        print(f"Unknown command: {cmd}")
        sys.exit(1)


if __name__ == "__main__":
    main()
