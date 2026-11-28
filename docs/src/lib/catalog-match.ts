export function normalizeSearch(value: string) {
  return value
    .normalize("NFKC")
    .toLowerCase()
    .replace(/[^\p{L}\p{N}]+/gu, " ")
    .trim();
}

export function matchRank(query: string, names: string[], terms: string[]) {
  const normalized = names.map(normalizeSearch);
  if (normalized.includes(query)) return 0;
  if (normalized.some((name) => name.startsWith(query))) return 1;
  const searchable = normalizeSearch([...names, ...terms].join(" "));
  return query.split(" ").every((word) => searchable.includes(word)) ? 2 : -1;
}
