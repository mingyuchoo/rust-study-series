#!/usr/bin/env python3
"""
TDD Workflow Traceability Verifier
코드베이스 전체에서 @trace 태그를 파싱하여 추적성 무결성을 검증하고,
추적성 매트릭스를 자동 생성한다.

사용법:
  python verify_trace.py              # 추적성 검증
  python verify_trace.py --matrix     # 추적성 매트릭스 생성
  python verify_trace.py --fix        # 누락된 trace_map 자동 복구
"""

import json
import os
import re
import sys
from datetime import datetime
from collections import defaultdict


# ---------------------------------------------------------------------------
# @trace 태그 파서
# ---------------------------------------------------------------------------

TRACE_PATTERNS = {
    "spec":      re.compile(r"@trace\s+SPEC:\s*(.+)"),
    "spec_bare": re.compile(r"@trace\s+(SPEC-\d{3})"),
    "prd":       re.compile(r"@trace\s+PRD:\s*(.+)"),
    "fr":        re.compile(r"@trace\s+FR:\s*(.+)"),
    "tc":        re.compile(r"@trace\s+TC:\s*(.+)"),
    "file_type": re.compile(r"@trace\s+file-type:\s*(.+)"),
    "scenario":  re.compile(r"@trace\s+scenario:\s*(.+)"),
}

CODE_EXTENSIONS = {".py", ".ts", ".js", ".tsx", ".jsx", ".go", ".rs", ".java", ".kt", ".rb"}


def find_source_files(root, dirs=("tests", "src", "lib", "app")):
    """프로젝트에서 소스/테스트 파일을 찾는다."""
    files = []
    for d in dirs:
        target = os.path.join(root, d)
        if not os.path.isdir(target):
            continue
        for dirpath, _, filenames in os.walk(target):
            for fn in filenames:
                ext = os.path.splitext(fn)[1]
                if ext in CODE_EXTENSIONS:
                    files.append(os.path.join(dirpath, fn))
    return files


def parse_trace_tags(filepath):
    """파일에서 @trace 태그를 추출한다."""
    result = {
        "filepath": filepath,
        "file_type": None,
        "specs": [],
        "prds": [],
        "frs": [],
        "tcs": [],
        "symbols": [],
    }

    try:
        with open(filepath, "r", encoding="utf-8") as f:
            content = f.read()
    except (UnicodeDecodeError, FileNotFoundError):
        return result

    lines = content.split("\n")

    # 파일 수준 태그 (상단 헤더 블록)
    for line in lines[:30]:
        m = TRACE_PATTERNS["file_type"].search(line)
        if m:
            result["file_type"] = m.group(1).strip()

        m = TRACE_PATTERNS["spec_bare"].search(line)
        if m:
            spec_id = m.group(1).strip()
            if spec_id not in result["specs"]:
                result["specs"].append(spec_id)

        m = TRACE_PATTERNS["spec"].search(line)
        if m:
            for s in m.group(1).split(","):
                s = s.strip()
                if s and s not in result["specs"]:
                    result["specs"].append(s)

        m = TRACE_PATTERNS["prd"].search(line)
        if m:
            for p in m.group(1).split(","):
                p = p.strip()
                if p and p not in result["prds"]:
                    result["prds"].append(p)

        m = TRACE_PATTERNS["fr"].search(line)
        if m:
            for fr in m.group(1).split(","):
                fr = fr.strip()
                if fr and fr not in result["frs"]:
                    result["frs"].append(fr)

    # 함수/클래스 수준 태그
    # Python: def/class 직전의 docstring에서 추출
    # JS/TS: JSDoc 블록에서 추출
    current_symbol = None
    in_docstring = False
    symbol_traces = {}

    for i, line in enumerate(lines):
        stripped = line.strip()

        # 심볼 감지
        sym_match = re.match(
            r"(?:export\s+)?(?:async\s+)?(?:def|function|class)\s+(\w+)", stripped
        )
        if sym_match:
            current_symbol = sym_match.group(1)
            if current_symbol not in symbol_traces:
                symbol_traces[current_symbol] = {"specs": [], "tcs": [], "frs": []}

        # @trace 태그가 있는 줄
        if "@trace" in line and current_symbol:
            m = TRACE_PATTERNS["tc"].search(line)
            if m:
                for tc in m.group(1).split(","):
                    tc = tc.strip()
                    if tc:
                        symbol_traces[current_symbol]["tcs"].append(tc)
                        if tc not in result["tcs"]:
                            result["tcs"].append(tc)

            m = TRACE_PATTERNS["fr"].search(line)
            if m:
                for fr in m.group(1).split(","):
                    fr = fr.strip()
                    if fr:
                        symbol_traces[current_symbol]["frs"].append(fr)

            m = TRACE_PATTERNS["spec"].search(line)
            if m:
                for s in m.group(1).split(","):
                    s = s.strip()
                    if s:
                        symbol_traces[current_symbol]["specs"].append(s)

    result["symbols"] = symbol_traces
    return result


# ---------------------------------------------------------------------------
# PRD / SPEC 문서 파서
# ---------------------------------------------------------------------------

def parse_prd_frs(prd_path):
    """PRD 문서에서 FR 목록을 추출한다."""
    frs = {}
    try:
        with open(prd_path, "r", encoding="utf-8") as f:
            content = f.read()
    except (FileNotFoundError, UnicodeDecodeError):
        return frs

    prd_id = os.path.splitext(os.path.basename(prd_path))[0]

    for m in re.finditer(r"[-*]\s+(FR-\d+):\s*(.+)", content):
        fr_id = m.group(1)
        title = m.group(2).strip()
        frs[fr_id] = {"title": title, "prd": prd_id}

    return frs


def parse_spec_tcs(spec_path):
    """SPEC 문서에서 TC->FR 매핑을 추출한다."""
    tcs = {}
    try:
        with open(spec_path, "r", encoding="utf-8") as f:
            content = f.read()
    except (FileNotFoundError, UnicodeDecodeError):
        return tcs

    spec_id = os.path.splitext(os.path.basename(spec_path))[0]

    # 마크다운 테이블에서 TC 행 추출
    # | TC-1 | ... | ... | ... | unit | FR-1 |
    for m in re.finditer(
        r"\|\s*(TC-\d+)\s*\|[^|]*\|[^|]*\|[^|]*\|[^|]*\|\s*(FR-\d+(?:\s*,\s*FR-\d+)*)\s*\|",
        content,
    ):
        tc_id = m.group(1)
        fr_refs = [fr.strip() for fr in m.group(2).split(",")]
        tcs[f"{spec_id}/{tc_id}"] = {"spec": spec_id, "frs": fr_refs}

    # 정방향 추적 테이블에서 SPEC->FR 매핑
    spec_frs = []
    for m in re.finditer(r"\|\s*PRD-\d+\s*\|\s*(FR-\d+)\s*\|", content):
        spec_frs.append(m.group(1))

    return {"tcs": tcs, "frs": spec_frs, "spec_id": spec_id}


# ---------------------------------------------------------------------------
# 검증 로직
# ---------------------------------------------------------------------------

def verify_traceability(root="."):
    """전체 추적성을 검증한다."""
    issues = []
    stats = {
        "total_frs": 0,
        "covered_frs": 0,
        "total_tcs": 0,
        "total_impl_symbols": 0,
        "untraced_symbols": 0,
    }

    # 1. PRD에서 FR 수집
    prd_dir = os.path.join(root, "docs", "prd")
    all_frs = {}  # {prd_id: {fr_id: {title, specs, tcs, impl}}}
    if os.path.isdir(prd_dir):
        for fn in sorted(os.listdir(prd_dir)):
            if fn.startswith("PRD-") and fn.endswith(".md"):
                prd_id = fn[:-3]
                frs = parse_prd_frs(os.path.join(prd_dir, fn))
                all_frs[prd_id] = {}
                for fr_id, info in frs.items():
                    all_frs[prd_id][fr_id] = {
                        "title": info["title"],
                        "specs": [],
                        "tcs": [],
                        "test_files": [],
                        "impl_files": [],
                        "impl_symbols": [],
                    }
                    stats["total_frs"] += 1

    # 2. SPEC에서 TC->FR 매핑 수집
    spec_dir = os.path.join(root, "docs", "spec")
    all_tcs = {}
    if os.path.isdir(spec_dir):
        for fn in sorted(os.listdir(spec_dir)):
            if fn.startswith("SPEC-") and fn.endswith(".md"):
                parsed = parse_spec_tcs(os.path.join(spec_dir, fn))
                spec_id = parsed["spec_id"]

                # SPEC->FR 매핑
                for prd_id, frs in all_frs.items():
                    for fr_id in parsed["frs"]:
                        if fr_id in frs:
                            if spec_id not in frs[fr_id]["specs"]:
                                frs[fr_id]["specs"].append(spec_id)

                # TC->FR 매핑
                for tc_key, tc_info in parsed["tcs"].items():
                    all_tcs[tc_key] = tc_info
                    stats["total_tcs"] += 1
                    for prd_id, frs in all_frs.items():
                        for fr_ref in tc_info["frs"]:
                            if fr_ref in frs:
                                if tc_key not in frs[fr_ref]["tcs"]:
                                    frs[fr_ref]["tcs"].append(tc_key)

    # 3. 코드 파일에서 @trace 태그 수집
    source_files = find_source_files(root)
    all_file_traces = []
    for fp in source_files:
        trace = parse_trace_tags(fp)
        all_file_traces.append(trace)
        rel_path = os.path.relpath(fp, root)

        for sym_name, sym_info in trace["symbols"].items():
            stats["total_impl_symbols"] += 1

            has_trace = bool(sym_info["frs"] or sym_info["tcs"] or sym_info["specs"])
            if not has_trace and trace["file_type"] == "impl":
                stats["untraced_symbols"] += 1
                issues.append(
                    f"WARN: 추적태그 없는 구현 함수: {rel_path}::{sym_name}"
                )

            # FR 매핑 역추적
            for fr_ref in sym_info["frs"]:
                # fr_ref 형식: "PRD-001/FR-1" 또는 "FR-1"
                if "/" in fr_ref:
                    prd_part, fr_part = fr_ref.split("/", 1)
                    if prd_part in all_frs and fr_part in all_frs[prd_part]:
                        entry = all_frs[prd_part][fr_part]
                        if rel_path not in entry["impl_files"]:
                            entry["impl_files"].append(rel_path)
                        if sym_name not in entry["impl_symbols"]:
                            entry["impl_symbols"].append(sym_name)

        # 테스트 파일 -> FR 매핑
        if trace["file_type"] == "test":
            for fr_ref in trace["frs"]:
                for prd_id, frs in all_frs.items():
                    fr_key = fr_ref.split("/")[-1] if "/" in fr_ref else fr_ref
                    if fr_key in frs:
                        if rel_path not in frs[fr_key]["test_files"]:
                            frs[fr_key]["test_files"].append(rel_path)

    # 4. 누락 검사
    for prd_id, frs in all_frs.items():
        for fr_id, info in frs.items():
            if not info["specs"]:
                issues.append(f"ERROR: SPEC 없는 FR: {prd_id}/{fr_id} \"{info['title']}\"")
            elif not info["tcs"]:
                issues.append(f"ERROR: TC 없는 FR: {prd_id}/{fr_id} \"{info['title']}\"")
            elif not info["impl_files"]:
                issues.append(f"ERROR: 구현 없는 FR: {prd_id}/{fr_id} \"{info['title']}\"")
            else:
                stats["covered_frs"] += 1

    # TC에 FR 매핑이 없는 경우
    for fp_trace in all_file_traces:
        if fp_trace["file_type"] == "test":
            for sym_name, sym_info in fp_trace["symbols"].items():
                if sym_info["tcs"] and not sym_info["frs"]:
                    rel = os.path.relpath(fp_trace["filepath"], root)
                    issues.append(
                        f"WARN: FR 매핑 없는 TC: {rel}::{sym_name} (TC: {sym_info['tcs']})"
                    )

    return {
        "issues": issues,
        "stats": stats,
        "frs": all_frs,
        "tcs": all_tcs,
        "file_traces": all_file_traces,
    }


# ---------------------------------------------------------------------------
# 추적성 매트릭스 생성
# ---------------------------------------------------------------------------

def generate_matrix(result, root="."):
    """docs/traceability-matrix.md 를 생성한다."""
    now = datetime.now().strftime("%Y-%m-%d %H:%M")
    lines = [
        "# 추적성 매트릭스",
        "",
        f"최종 갱신: {now}",
        "",
        "## 정방향 추적 (요구사항 -> 구현)",
        "",
        "| PRD | FR ID | FR 제목 | SPEC | TC | 테스트 파일 | 구현 파일 | 구현 심볼 | 상태 |",
        "|-----|-------|--------|------|-----|-----------|----------|----------|------|",
    ]

    total_fr = 0
    covered_fr = 0
    total_tc = 0
    passed_tc = 0

    for prd_id in sorted(result["frs"].keys()):
        frs = result["frs"][prd_id]
        for fr_id in sorted(frs.keys()):
            info = frs[fr_id]
            total_fr += 1
            tc_count = len(info["tcs"])
            total_tc += tc_count

            has_all = bool(info["specs"] and info["tcs"] and info["impl_files"])
            status = "PASS" if has_all else "INCOMPLETE"
            if has_all:
                covered_fr += 1
                passed_tc += tc_count

            specs = ", ".join(info["specs"]) if info["specs"] else "-"
            tcs = ", ".join(tc.split("/")[-1] for tc in info["tcs"]) if info["tcs"] else "-"
            test_fs = ", ".join(info["test_files"]) if info["test_files"] else "-"
            impl_fs = ", ".join(info["impl_files"]) if info["impl_files"] else "-"
            impl_syms = ", ".join(info["impl_symbols"]) if info["impl_symbols"] else "-"

            lines.append(
                f"| {prd_id} | {fr_id} | {info['title'][:30]} | {specs} | {tcs} | {test_fs} | {impl_fs} | {impl_syms} | {status} |"
            )

    # 역방향 추적
    lines += [
        "",
        "## 역방향 추적 (구현 -> 요구사항)",
        "",
        "| 구현 파일 | 심볼 | SPEC | TC | FR | PRD | 상태 |",
        "|----------|------|------|-----|-----|-----|------|",
    ]

    for ft in result["file_traces"]:
        if ft["file_type"] != "impl":
            continue
        rel = os.path.relpath(ft["filepath"], root)
        for sym_name, sym_info in sorted(ft["symbols"].items()):
            if not (sym_info["frs"] or sym_info["tcs"] or sym_info["specs"]):
                lines.append(f"| {rel} | {sym_name} | - | - | - | - | UNTRACED |")
                continue

            specs = ", ".join(sym_info["specs"]) if sym_info["specs"] else "-"
            tcs = ", ".join(sym_info["tcs"]) if sym_info["tcs"] else "-"
            frs = ", ".join(sym_info["frs"]) if sym_info["frs"] else "-"
            prd = "-"
            for fr in sym_info["frs"]:
                if "/" in fr:
                    prd = fr.split("/")[0]
                    break
            status = "OK" if sym_info["frs"] else "NO_FR"
            lines.append(f"| {rel} | {sym_name} | {specs} | {tcs} | {frs} | {prd} | {status} |")

    # 커버리지 요약
    prd_summary = defaultdict(lambda: {"total_fr": 0, "covered_fr": 0, "specs": set(), "tcs": 0, "passed": 0})
    for prd_id, frs in result["frs"].items():
        for fr_id, info in frs.items():
            s = prd_summary[prd_id]
            s["total_fr"] += 1
            has_all = bool(info["specs"] and info["tcs"] and info["impl_files"])
            if has_all:
                s["covered_fr"] += 1
            s["specs"].update(info["specs"])
            s["tcs"] += len(info["tcs"])
            if has_all:
                s["passed"] += len(info["tcs"])

    lines += [
        "",
        "## 커버리지 요약",
        "",
        "| PRD | 전체 FR | 커버된 FR | SPEC 수 | TC 수 | 커버리지 |",
        "|-----|--------|----------|--------|-------|---------|",
    ]
    for prd_id in sorted(prd_summary.keys()):
        s = prd_summary[prd_id]
        cov = f"{s['covered_fr']/s['total_fr']*100:.0f}%" if s["total_fr"] > 0 else "0%"
        lines.append(
            f"| {prd_id} | {s['total_fr']} | {s['covered_fr']} | {len(s['specs'])} | {s['tcs']} | {cov} |"
        )

    # 미추적 항목
    lines += ["", "## 미추적 항목 (경고)", ""]
    if result["issues"]:
        for issue in result["issues"]:
            lines.append(f"- {issue}")
    else:
        lines.append("없음")

    lines.append("")

    matrix_path = os.path.join(root, "docs", "traceability-matrix.md")
    os.makedirs(os.path.dirname(matrix_path), exist_ok=True)
    with open(matrix_path, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))

    return matrix_path


# ---------------------------------------------------------------------------
# trace_map 복구
# ---------------------------------------------------------------------------

def rebuild_trace_map(result, root="."):
    """코드의 @trace 태그로부터 registry.json 의 trace_map 을 재구성한다."""
    reg_path = os.path.join(root, "docs", "registry.json")
    if os.path.exists(reg_path):
        with open(reg_path, "r", encoding="utf-8") as f:
            registry = json.load(f)
    else:
        registry = {"last_prd_id": 0, "last_spec_id": 0, "entries": [], "trace_map": {}}

    new_map = {}
    for prd_id, frs in result["frs"].items():
        new_map[prd_id] = {"fr": {}}
        for fr_id, info in frs.items():
            has_all = bool(info["specs"] and info["tcs"] and info["impl_files"])
            new_map[prd_id]["fr"][fr_id] = {
                "title": info["title"],
                "specs": info["specs"],
                "test_cases": info["tcs"],
                "test_files": info["test_files"],
                "impl_files": info["impl_files"],
                "impl_symbols": info["impl_symbols"],
                "status": "passed" if has_all else "draft",
            }

    registry["trace_map"] = new_map
    with open(reg_path, "w", encoding="utf-8") as f:
        json.dump(registry, f, indent=2, ensure_ascii=False)

    return reg_path


# ---------------------------------------------------------------------------
# 보고서 출력
# ---------------------------------------------------------------------------

def print_report(result):
    """추적성 검증 보고서를 출력한다."""
    s = result["stats"]
    issues = result["issues"]

    print("=== 추적성 검증 보고서 ===")
    print()

    # 정방향 추적
    print("[정방향 추적]")
    for prd_id in sorted(result["frs"].keys()):
        frs = result["frs"][prd_id]
        for fr_id in sorted(frs.keys()):
            info = frs[fr_id]
            specs_ok = "OK" if info["specs"] else "MISSING"
            tcs_ok = f"OK ({len(info['tcs'])}개)" if info["tcs"] else "MISSING"
            impl_ok = "OK" if info["impl_files"] else "MISSING"

            print(f"  {prd_id}/{fr_id} \"{info['title'][:40]}\"")
            print(f"    -> SPEC: {', '.join(info['specs']) if info['specs'] else '(없음)'}  {specs_ok}")
            print(f"    -> TC:   {', '.join(info['tcs']) if info['tcs'] else '(없음)'}  {tcs_ok}")
            print(f"    -> IMPL: {', '.join(info['impl_files']) if info['impl_files'] else '(없음)'}  {impl_ok}")
            if info["impl_symbols"]:
                for sym in info["impl_symbols"]:
                    print(f"       {sym}()")
            print()

    # 역방향 추적
    print("[역방향 추적]")
    for ft in result["file_traces"]:
        if ft["file_type"] != "impl":
            continue
        rel = os.path.relpath(ft["filepath"])
        print(f"  {rel}")
        for sym_name, sym_info in sorted(ft["symbols"].items()):
            tcs = ", ".join(sym_info["tcs"]) if sym_info["tcs"] else "(없음)"
            frs = ", ".join(sym_info["frs"]) if sym_info["frs"] else "(없음)"
            status = "OK" if sym_info["frs"] else "UNTRACED"
            print(f"    {sym_name}()  -> TC: {tcs} -> FR: {frs}  {status}")
        print()

    # 누락 검사
    print("[누락 검사]")
    errors = [i for i in issues if i.startswith("ERROR")]
    warns = [i for i in issues if i.startswith("WARN")]

    if not issues:
        print("  문제 없음")
    else:
        for issue in errors:
            print(f"  {issue}")
        for issue in warns:
            print(f"  {issue}")

    print()
    coverage = f"{s['covered_frs']}/{s['total_frs']}" if s["total_frs"] > 0 else "0/0"
    pct = f"{s['covered_frs']/s['total_frs']*100:.0f}%" if s["total_frs"] > 0 else "N/A"
    completeness = "COMPLETE" if not errors else "INCOMPLETE"

    print(f"FR 커버리지: {coverage} ({pct})")
    print(f"TC 수: {s['total_tcs']}")
    print(f"구현 심볼: {s['total_impl_symbols']} (미추적: {s['untraced_symbols']})")
    print(f"추적성: {completeness}")

    return len(errors) == 0


# ---------------------------------------------------------------------------
# main
# ---------------------------------------------------------------------------

def main():
    root = "."
    do_matrix = "--matrix" in sys.argv
    do_fix = "--fix" in sys.argv

    result = verify_traceability(root)

    if do_fix:
        path = rebuild_trace_map(result, root)
        print(f"trace_map 복구 완료: {path}")

    if do_matrix:
        path = generate_matrix(result, root)
        print(f"추적성 매트릭스 생성: {path}")

    ok = print_report(result)

    if do_matrix:
        generate_matrix(result, root)

    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
