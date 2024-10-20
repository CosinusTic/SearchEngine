###TODO: 

## PHASE 1: Basic URL matching
- Collect URL base in a .TXT file
- Scraping function
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

