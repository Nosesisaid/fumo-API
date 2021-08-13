var mongoose = require("mongoose");
mongoose.connect(process.env.MONGOURL, {
  useNewUrlParser: true,
  useUnifiedTopology: true,
})
const db = mongoose.connection
db.on("open", _ => console.log("database ready"))

