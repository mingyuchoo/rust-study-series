interface Props {
  from: string;
  to: string;
  onFromChange: (date: string) => void;
  onToChange: (date: string) => void;
}

export function DateRangeFilter({
  from,
  to,
  onFromChange,
  onToChange,
}: Props) {
  return (
    <div className="date-range-filter">
      <label>
        시작일
        <input
          type="date"
          value={from}
          onChange={(e) => onFromChange(e.target.value)}
        />
      </label>
      <label>
        종료일
        <input
          type="date"
          value={to}
          onChange={(e) => onToChange(e.target.value)}
        />
      </label>
    </div>
  );
}
