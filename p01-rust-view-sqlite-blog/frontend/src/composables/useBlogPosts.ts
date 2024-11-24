import { ref } from 'vue';

interface BlogPost {
  id: number;
  title: string;
  date: number;
  body: string;
}

const postList = ref<BlogPost[]>([]);

export default function useBlogPosts() {
  return postList;
}
