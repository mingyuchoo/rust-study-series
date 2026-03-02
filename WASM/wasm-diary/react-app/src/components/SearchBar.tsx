import { useEffect, useState } from "react";

interface Props {
  onSearch: (keyword: string) => void;
}

export function SearchBar({ onSearch }: Props) {
  const [keyword, setKeyword] = useState("");

  useEffect(() => {
    const timer = setTimeout(() => {
      onSearch(keyword);
    }, 300);
    return () => clearTimeout(timer);
  }, [keyword, onSearch]);

  return (
    <div className="search-bar">
      <input
        type="text"
        value={keyword}
        onChange={(e) => setKeyword(e.target.value)}
        placeholder="키워드로 검색..."
        className="search-input"
      />
    </div>
  );
}
