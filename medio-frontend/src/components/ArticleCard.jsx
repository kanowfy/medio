import { Card } from "flowbite-react"
import { Link } from "react-router-dom";

const ArticleCard = (props) => {
    return (
        <Card
            className="max-w-sm"
            imgSrc="./assets/image-1.jpg" horizontal
        >
            <Link to={`/article/${props.id}`}>
                <h5 className="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
                    {props.title}
                </h5>

            </Link>            
            <p className="font-normal text-gray-700 dark:text-gray-400 text-sm">
                {props.content.split(' ').slice(0, 20).join(' ')}...
                <Link to={`/article/${props.id}`} className="text-teal-700 font-medium">Read more</Link>
            </p>
        </Card>
    )
}

export default ArticleCard;