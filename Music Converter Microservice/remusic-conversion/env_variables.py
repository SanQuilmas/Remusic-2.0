# Prod Env
KAFKA_BROKER = 'broker:9092'
POST_URL = 'http://backend:8080/sheet/'

# Local Dev Env
# KAFKA_BROKER = 'localhost:9092'
# POST_URL = 'http://localhost:8080/sheet/'
#   - ./remusic/temp:/app/temp

# Kafka Info
INPUT_TOPIC = 'newly_created'
INPROGRESS_TOPIC = 'jobs_inprogress'
DONE_TOPIC = 'newly_updated'
GROUP_ID = 'oemer-wrapper'

OEMER_BINARY = './oemer'