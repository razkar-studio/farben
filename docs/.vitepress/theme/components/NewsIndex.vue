<template>
    <div class="news">
        <article v-for="post in posts" :key="post.url" class="post">
            <a :href="withBase(post.url)" class="post-link">
                <h2 class="post-title">{{ post.title }}</h2>
                <span class="post-date">{{ formatDate(post.date) }}</span>
            </a>
        </article>
        <p v-if="posts.length === 0" class="empty">
            No posts yet. Check back soon.
        </p>
    </div>
</template>

<script setup>
import { data as posts } from "../../../news/news.data.ts";
import { withBase } from "vitepress";

function formatDate(iso) {
    if (!iso) return "";
    const d = new Date(iso);
    if (isNaN(d.getTime())) return iso;
    return d.toLocaleDateString("en-US", {
        year: "numeric",
        month: "long",
        day: "numeric",
    });
}
</script>

<style scoped>
.news {
    max-width: 740px;
    margin: 0 auto;
    padding: 2rem 0 4rem;
}

.post {
    padding: 1.25rem 0;
    border-bottom: 1px solid var(--vp-c-divider);
}

.post:last-child {
    border-bottom: none;
}

.post-link {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 1rem;
    text-decoration: none;
    color: inherit;
    transition: color 0.15s;
}

.post-link:hover .post-title {
    color: var(--vp-c-brand-1);
}

.post-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--vp-c-text-1);
    margin: 0;
    border: none;
    padding: 0;
    transition: color 0.15s;
}

.post-date {
    font-size: 0.9rem;
    color: var(--vp-c-text-3);
    white-space: nowrap;
    flex-shrink: 0;
}

.empty {
    color: var(--vp-c-text-2);
    font-style: italic;
    text-align: center;
    padding: 3rem 0;
}
</style>
