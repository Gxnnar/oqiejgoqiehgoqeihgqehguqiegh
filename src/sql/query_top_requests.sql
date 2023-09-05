SELECT request_origin,
    COUNT(*) counter
FROM requests
WHERE response_status == 200
    AND request_origin NOT LIKE '%cdn%'
    AND request_origin NOT LIKE '%api%'
GROUP BY request_origin
ORDER BY counter DESC
LIMIT ?1;