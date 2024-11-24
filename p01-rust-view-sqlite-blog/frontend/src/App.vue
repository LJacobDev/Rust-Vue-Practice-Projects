<script setup lang="ts">
import useBlogPosts from '@/composables/useBlogPosts';

const postList = useBlogPosts();
</script>

<template>
  <main class="mx-auto text-center py-8 space-y-8">
    <h1 class="text-2xl">Popular Informative Blog</h1>
    <div v-if="postList.length != 0">
      <ul>
        <li v-for="post in postList" :key="post.id">{{ post.title }} hi</li>
      </ul>
    </div>
    <div v-else>
      <h2>No posts are available yet.  Please create one!</h2>
    </div>
    <button class=" border border-gray-600 rounded-lg bg-gray-200 py-2 px-4 hover:bg-gray-100">
      Create Post
    </button>
  </main>
</template>
