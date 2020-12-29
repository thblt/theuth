import { useQuery } from "react-query";

export function fetchProgNotions() {
    return fetch("/programmes/notions").then((res) => res.json());
}

export function fetchProgReperes() {
    return fetch("/programmes/reperes").then((res) => res.json());
}

export function fetchProgHLP() {
    return fetch("/programmes/hlp").then((res) => res.json());
}
