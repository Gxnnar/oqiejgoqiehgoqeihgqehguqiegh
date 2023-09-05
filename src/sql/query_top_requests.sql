SELECT request_origin,
    COUNT(*) counter
FROM requests
WHERE response_status == 200
    AND request_origin != 'unpkg.com'
    AND request_origin != 'upload.wikimedia.org'
    AND request_origin NOT LIKE '%cdn%'
    AND request_origin NOT LIKE '%api%'
    AND request_origin NOT LIKE '%asset%'
    AND request_origin NOT LIKE '%githubusercontent%'
GROUP BY request_origin
ORDER BY counter DESC
LIMIT ?1;