<script lang="ts">
    import { db } from "$lib/db";
    import { NoPosts, Posts } from "$lib";

    function checkPosts() {
        if (db.posts.length === 0) {
            return true;
        }
        return false;
    }

</script>

{#if checkPosts()}
    <NoPosts />
{:else}
    <Posts posts={db.posts} />
{/if}