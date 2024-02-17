metadata package = [ { namespace: "org.kny.medio.article", crate: "article_crate" } ]

namespace org.kny.medio.article

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U64

@wasmbus( actorReceive: true )
service Article {
  version: "0.1",
  operations: [ GetAllArticles, GetArticlesForAuthor, GetOneArticle, CreateArticle, UpdateArticle, DeleteArticle ]
}

operation GetAllArticles {
  output: ArticleList
}

operation GetArticlesForAuthor {
  input: U64,
  output: ArticleList
}

operation GetOneArticle {
  input: U64,
  output: GetOneArticleResponse
}

operation CreateArticle {
  input: CreateArticleRequest,
  output: CreateArticleResponse
}

operation UpdateArticle {
  input: UpdateArticleRequest,
  output: UpdateArticleResponse
}

operation DeleteArticle {
  input: U64,
  output: DeleteArticleResponse
}

structure GetOneArticleResponse {
    @required
    success: Boolean,
    reason: String,
    article: ArticleDto
}

structure CreateArticleRequest {
  @required
  authorId: U64,
  @required
  title: String,
  @required
  content: String
}

structure CreateArticleResponse {
  @required
  success: Boolean,
  reason: String,
  id: U64
}

structure UpdateArticleRequest {
  @required
  targetId: U64,
  title: String,
  content: String
}

structure UpdateArticleResponse {
  @required
  success: Boolean,
  reason: String
}

structure DeleteArticleResponse {
  @required
  success: Boolean,
  reason: String
}

structure ArticleDto {
  @required
  id: U64,
  @required
  authorId: U64,
  @required
  title: String,
  @required
  content: String,
  /// Time is represented using UNIX Timestamp, a.k.a number of seconds passed since epoch
  @required
  createdAt: U64,
  updatedAt: U64
}

list ArticleList {
  member: ArticleDto
}
