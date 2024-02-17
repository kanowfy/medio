import { useEffect, useState } from "react";
import ArticleCard from "../components/ArticleCard";
import Layout from "./Layout";
import articleService from "../services/articles";
import SubscribeBox from "../components/SubscribeBox";

const Home = () => {
    const [articles, setArticles] = useState([]);
    
    useEffect(() => {
        const fetchArticles = async () => {
            try {
                const articles = await articleService.getAll();
                console.log(articles);
                setArticles(articles);
            } catch(e) {
                console.error(e);
            }
        }

        fetchArticles();
    }, []);

    return (
        <Layout>
            <div className="min-h-screen">
                <div className="text-4xl font-bold mt-10 ml-10">Latest Articles</div>
                <div className="grid grid-cols-1 gap-12 md:grid-cols-2 xl:grid-cols-2 m-16">
                    {articles?.map(a => (
                        <ArticleCard id={a.id} title={a.title} content={a.content} key={a.id}/>
                    ))}
                </div>
            </div>
            <div className="fixed bottom-0 right-0 p-4">
                <SubscribeBox />
            </div>
        </Layout>
    )
}

export default Home;