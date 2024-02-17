import { useEffect, useState } from "react"
import { useParams } from "react-router-dom";
import articleService from "../services/articles";
import Layout from "./Layout";
import { Card } from "flowbite-react";
import timeUtils from "../utils/timeUtils";

const Article = () => {
    const params = useParams();
    const [article, setArticle] = useState({});

    useEffect(() => {
        const fetchArticle = async () => {
            try {
                const data = await articleService.getOne(params.articleId);
                setArticle(data.article);
            } catch (e) {
                console.error(e);
            }
        }

        fetchArticle();
    }, [params.articleId]);

    return (
        <Layout>
            <Card className="min-h-screen px-10 py-5 mx-44 my-5">
                <div className="text-3xl font-bold">{article.title}</div>
                <div className="mb-5">{timeUtils.formatUnixTime(article.createdAt)}</div>
                <div>{article.content}</div>
            </Card>
        </Layout>
    )
}

export default Article;