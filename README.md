###TODO: 

## PHASE 1: Web crawling
- From a small base of urls, recursively visit urls in pages and collect them
- Store them in a .txt file which will then serve as base (a small "internet")

## PHASE 1: Basic URL matching
- Scraping functions (scrape various tags robustly)
- Assign score to urls (frequency matching, tag value)

## PHASE 2: Implement into custom structures
- Implement the BST storage
    - Give a score to a certain url
    - Store the url in the tree along with its score (acting as node value)
- Simply output the tree and it will be the search results (probably BFS or suffix DFS)


## Phase 3: Going further
- Result Ranking: using Levenshtein distance
- Parallelization (use tokio or rayon) to parallelize scraping and matching operations
- Results caching


