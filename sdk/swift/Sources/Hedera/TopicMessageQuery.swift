import Foundation

/// Query a stream of Hedera Consensus Service (HCS)
/// messages for an HCS Topic via a specific (possibly open-ended) time range.
public final class TopicMessageQuery: MirrorQuery<[TopicMessage]> {
    /// Create a new `TopicMessageQuery`.
    public init(
        topicId: TopicId? = nil,
        startTime: Date? = nil,
        endTime: Date? = nil,
        limit: UInt64 = 0
    ) {
        self.topicId = topicId
        self.startTime = startTime
        self.endTime = endTime
        self.limit = limit
    }

    /// The topic ID to retrieve messages for.
    public var topicId: TopicId?

    /// Include messages which reached consensus on or after this time.
    /// Defaults to the current time.
    public var startTime: Date?

    /// Include messages which reached consensus before this time.
    public var endTime: Date?

    /// The maximum number of message to receive before stopping.
    public var limit: UInt64

    /// Sets topic ID to retrieve messages for.
    @discardableResult
    public func topicId(_ topicId: TopicId) -> Self {
        self.topicId = topicId

        return self
    }

    /// Set to include messages which reached consensus on or after this time.
    /// Defaults to the current time.
    @discardableResult
    public func startTime(_ startTime: Date) -> Self {
        self.startTime = startTime

        return self
    }

    /// Set to include messages which reached consensus before this time.
    @discardableResult
    public func endTime(_ endTime: Date) -> Self {
        self.endTime = endTime

        return self
    }

    /// Sets the maximum number of messages to be returned, before closing the subscription.
    /// Defaults to _unlimited_.
    @discardableResult
    public func limit(_ limit: UInt64) -> Self {
        self.limit = limit

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case topicId
        case startTime
        case endTime
        case limit
    }

    public override func encode(to encoder: Encoder) throws {
        var container: KeyedEncodingContainer<TopicMessageQuery.CodingKeys> = encoder.container(
            keyedBy: CodingKeys.self)

        try container.encodeIfPresent(topicId, forKey: .topicId)
        try container.encodeIfPresent(startTime?.unixTimestampNanos, forKey: .startTime)
        try container.encodeIfPresent(endTime?.unixTimestampNanos, forKey: .endTime)

        if limit != 0 {
            try container.encode(limit, forKey: .limit)
        }

        try super.encode(to: encoder)
    }
}