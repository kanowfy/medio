const formatUnixTime = (timestamp) => {
    return new Date(timestamp * 1000).toDateString();
}

export default { formatUnixTime };