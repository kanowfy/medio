use article::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;

#[allow(dead_code)]
mod article;
mod store;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Article)]
struct ArticleActor {}

#[async_trait]
impl Article for ArticleActor {
    async fn get_all_articles(&self, ctx: &Context) -> RpcResult<ArticleList> {
        info!("Received get all articles request");
        Ok(match store::get_all_articles(ctx).await {
            Ok(list) => list,
            Err(_) => vec![]
        })
    }

    async fn get_articles_for_author(&self, ctx: &Context, id: &u64) -> RpcResult<ArticleList> {
        info!("Received get articles for author request");
        Ok(match store::get_articles_for_author(ctx, id).await {
            Ok(list) => list,
            Err(_) => vec![]
        })
    }

    async fn get_one_article(&self, ctx: &Context, id: &u64) -> RpcResult<GetOneArticleResponse> {
        info!("Received get article request with id {}", id);
        Ok(match store::get_one_article(ctx, id).await {
            Ok(article) => GetOneArticleResponse {
                success: true,
                article: Some(article),
                reason: None
            },
            Err(e) => GetOneArticleResponse {
                success: false,
                reason: Some(format!("{}", e)),
                article: None
            }
        })
    }

    //TODO: publish event to article topic
    async fn create_article(&self, ctx: &Context, article: &CreateArticleRequest) -> RpcResult<CreateArticleResponse> {
        info!("Received create article request: {:?}", article);
        Ok(match store::create_article(ctx, article).await {
            Ok(a) => {
                CreateArticleResponse {
                    success: true,
                    id: Some(a.id),
                    reason: None
                }
            },
            Err(e) => CreateArticleResponse {
                success: false,
                reason: Some(format!("{}", e)),
                id: None
            }
        })
    }

    async fn update_article(&self, ctx: &Context, article: &UpdateArticleRequest) -> RpcResult<UpdateArticleResponse> {
        info!("Received update article request: {:?}", article);
        Ok(match store::update_article(ctx, article).await {
            Ok(()) => UpdateArticleResponse {
                success: true,
                reason: None
            },
            Err(e) => UpdateArticleResponse {
                success: false,
                reason: Some(format!("{}", e)),
            }
        })
    }

    async fn delete_article(&self, ctx: &Context, id: &u64) -> RpcResult<DeleteArticleResponse> {
        info!("Received delete article request with id {}", id);
        Ok(match store::delete_article(ctx, id).await {
            Ok(()) => DeleteArticleResponse {
                success: true,
                reason: None
            },
            Err(e) => DeleteArticleResponse {
                success: false,
                reason: Some(format!("{}", e)),
            }
        })
    }
}

