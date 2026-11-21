"use client";

import { useSyncExternalStore } from "react";

import { type Accent, accents, accentStorageKey } from "@/lib/accent";

function readAccent(): Accent {
  return accents.find((value) => value === document.documentElement.dataset.accent) ?? "neutral";
}

function subscribe(onChange: () => void) {
  const observer = new MutationObserver(onChange);
  observer.observe(document.documentElement, {
    attributes: true,
    attributeFilter: ["data-accent"],
  });
  return () => observer.disconnect();
}

export function useAccent(): Accent {
  return useSyncExternalStore(subscribe, readAccent, () => "neutral");
}

export function selectAccent(value: Accent) {
  document.documentElement.dataset.accent = value;
  try {
    localStorage.setItem(accentStorageKey, value);
  } catch {
    // The current page still uses the chosen color when storage is unavailable.
  }
}
